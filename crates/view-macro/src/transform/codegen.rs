use {
    crate::transform::{ChildrenStructure, Element, ViewNodes},
    proc_macro2::TokenStream,
};
use {
    crate::transform::{ForExpr, ViewNode},
    quote::{format_ident, quote},
};

impl Element {
    pub fn to_tokens(&self) -> TokenStream {
        let path = &self.path;

        let children = {
            let each: Vec<_> =
                self.children.iter().map(|c| c.to_tokens()).collect();

            if let ChildrenStructure::ConsList = self.children_structure {
                quote! { list![#(#each),*] }
            } else {
                quote! { #(#each),* }
            }
        };

        let mut output = quote!( #path(#children) );

        for attr in &self.attributes {
            let method = format_ident!("with_{}", attr.key);
            let val = &attr.value;
            output = match val {
                Some(v) => quote!( #output.#method(#v) ),
                None => quote!( #output.#method() ),
            };
        }

        output
    }
}

impl ViewNode {
    pub fn to_tokens(&self) -> TokenStream {
        match self {
            ViewNode::Element(element) => element.to_tokens(),
            ViewNode::Block(expr) => quote! { #expr },
            ViewNode::ForExpr(for_expr) => for_expr.to_tokens(),
        }
    }
}

impl ViewNodes {
    pub fn to_tokens(&self) -> TokenStream {
        let body: Vec<_> = self.0.iter().map(|i| i.to_tokens()).collect();
        if body.len() > 1 {
            quote! {
                list![ #(#body),* ]
            }
        } else {
            quote! {
                #(#body),*
            }
        }
    }
}

impl ForExpr {
    pub fn to_tokens(&self) -> TokenStream {
        let pat = &self.pat;
        let expr = &self.expr;
        let body: Vec<_> = self.body.iter().map(|i| i.to_tokens()).collect();

        if body.len() > 1 {
            quote! {
                #expr.map(move |#pat| list![ #(#body),* ]).into_blueprint()
            }
        } else {
            quote! {
                #expr.map(move |#pat| { #(#body),* }).into_blueprint()
            }
        }
    }
}
