use {
    proc_macro::TokenStream,
    proc_macro2::{Span, TokenStream as TokenStream2},
    proc_macro_error2::{abort, emit_call_site_warning, proc_macro_error},
    quote::quote,
    syn::{parse_macro_input, Data, DataStruct, DeriveInput, Field, Path},
};

#[proc_macro_derive(
    ToDefaultUi,
    attributes(container, to_ui, props, wrap, ui_exclude)
)]
#[proc_macro_error]
pub fn to_default_ui(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);
    transform_type(&mut input).into()
}

fn transform_type(input: &mut DeriveInput) -> TokenStream2 {
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) =
        input.generics.split_for_impl();

    let mut container = None;

    for attr in &input.attrs {
        if attr.path().is_ident("container") {
            let id: Path = attr.parse_args().unwrap_or_else(|_| {
                abort!(attr, "The `container` attribute requires a path, e.g., #[container(MyContainer)]");
            });
            container = Some(id);
        }
    }

    let Some(container) = container else {
        abort!(Span::call_site(), "`container` must be specified (so there's a place where to put the ui items)")
    };

    let (mut ui_type, conversion_expr) = match &input.data {
        Data::Struct(data_struct) => transform_struct(data_struct, &container),
        Data::Enum(_) => unimplemented!("not implemented for enums"),
        Data::Union(_) => unimplemented!("not implemented for unions"),
    };

    if ui_type.is_empty() {
        ui_type = quote! {()};
    }

    let expanded = quote! {
        impl #impl_generics ToDefaultUi for #name #ty_generics #where_clause {
            type Ui = #ui_type;

            fn to_default_ui(&self) -> Self::Ui {
                #conversion_expr
            }
        }
    };
    expanded
}

fn transform_struct(
    data: &DataStruct,
    container: &Path,
) -> (TokenStream2, TokenStream2) {
    let fields: Vec<_> = data
        .fields
        .iter()
        .enumerate()
        .filter_map(transform_field)
        .collect();
    let types: Vec<_> = fields.iter().map(|(ty, _)| ty).collect();
    let types = if types.is_empty() {
        quote!(())
    } else {
        transform_tuple_list(types.into_iter())
    };

    let conversions = fields.iter().map(|(_, con)| con);
    let conversions = transform_tuple_list(conversions);

    let ui_type = quote! { #container<#types> };
    let conversion_expr = quote! {
        <#container<_> as Edit<_>>::edit(
            &#conversions
        )
    };

    (ui_type, conversion_expr)
}

fn transform_field(
    (field_index, data): (usize, &Field),
) -> Option<(TokenStream2, TokenStream2)> {
    let field_access = if let Some(ident) = &data.ident {
        quote! { #ident }
    } else {
        quote! { #field_index }
    };
    let ty = &data.ty;

    for attr in &data.attrs {
        if attr.path().is_ident("ui_exclude") {
            return None;
        }
    }

    let mut to_ui_override: Option<Path> = None;
    for attr in &data.attrs {
        if attr.path().is_ident("to_ui") {
            let path = attr.parse_args();
            if let Ok(path) = path {
                to_ui_override = Some(path)
            } else {
                emit_call_site_warning!("Ignored: `to_ui` requires a Path e.g. `#[to_ui(Component)]`.")
            }
        }
    }

    let mut ui_type = quote! { <#ty as ToDefaultUi>::Ui };
    let mut conversion_expr =
        quote! { ToDefaultUi::to_default_ui(&self.#field_access) };

    // #[into_ui]
    // For overriding what component is used to edit this item.
    if let Some(editor) = to_ui_override {
        ui_type = quote! { #editor<#ty> };
        conversion_expr =
            quote! { <#editor<_> as Edit<_>>::edit(&self.#field_access) };
    }

    // #[props(foo = bar, ...)]
    // For adding properties in UI Composer's
    // fluid interface/builder style.
    for attr in &data.attrs {
        if attr.path().is_ident("props") {
            // Parse as comma separated list of "key = value"
            let nested = attr
                .parse_args_with(
                    syn::punctuated::Punctuated::<
                        syn::MetaNameValue,
                        syn::Token![,],
                    >::parse_terminated,
                )
                .expect("props must be key=value");

            for nv in nested {
                let name = &nv.path;
                let value = &nv.value;
                let method =
                    quote::format_ident!("with_{}", name.get_ident().unwrap());
                conversion_expr = quote! { #conversion_expr.#method(#value) };
            }
        }
    }

    // #[wrap(foo(bar = baz, quz = qux))]
    // Wraps the item in another component. Useful for container items like `flex`
    // which doesn't accept `LayoutItem`s directly, and instead asks for `FlexItem`s`.
    for attr in &data.attrs {
        if attr.path().is_ident("wrap") {
            let meta: syn::Meta =
                attr.parse_args().expect("wrap requires a component");

            let wrapper_path = &meta.path();
            conversion_expr = quote! { #wrapper_path(#conversion_expr) };
            ui_type = quote! { #wrapper_path<#ui_type> };

            if let syn::Meta::List(list) = &meta {
                let inner_props = list
                    .parse_args_with(
                        syn::punctuated::Punctuated::<
                            syn::MetaNameValue,
                            syn::Token![,],
                        >::parse_terminated,
                    )
                    .ok();

                if let Some(props) = inner_props {
                    for nv in props {
                        let name = &nv.path;
                        let value = &nv.value;
                        let method = quote::format_ident!(
                            "with_{}",
                            name.get_ident().unwrap()
                        );
                        conversion_expr =
                            quote! { #conversion_expr.#method(#value) };
                    }
                }
            }
        }
    }

    Some((ui_type, conversion_expr))
}

/// Transform a list of expressions into a tuple-based cons list.
fn transform_tuple_list<'a, I>(mut iter: I) -> TokenStream2
where
    I: Iterator<Item = &'a TokenStream2>,
{
    if let Some(first) = iter.next() {
        let rest = transform_tuple_list(iter);
        quote!((#first, #rest))
    } else {
        quote!(())
    }
}
