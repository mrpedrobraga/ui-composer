use super::IfExpr;

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
            let each: Vec<_> = self.children.iter().map(|c| c.to_tokens()).collect();

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
            ViewNode::IfExpr(if_expr) => if_expr.to_tokens(),
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

        /* Empty state */
        let empty_state = self.empty_state.as_ref().map(|body| {
            let body = body.iter().map(|i| i.to_tokens());
            quote! {
                .with_empty_state({
                    #(#body)*
                })
            }
        });

        if body.len() > 1 {
            quote! {
                #expr.for_of(move |#pat| list![ #(#body),* ]) #empty_state
            }
        } else {
            quote! {
                #expr.for_of(move |#pat| { #(#body),* }) #empty_state
            }
        }
    }
}

impl IfExpr {
    pub fn to_tokens(&self) -> TokenStream {
        let condition = &self.condition;
        let body: Vec<_> = self.body.iter().map(|i| i.to_tokens()).collect();

        let content = if body.len() > 1 {
            quote! { list![ #(#body),* ] }
        } else {
            quote! { { #(#body),* } }
        };

        if let Some(else_body) = &self.else_body {
            let else_body: Vec<_> = else_body.iter().map(ViewNode::to_tokens).collect();
            let else_content = if else_body.len() > 1 {
                quote! { list![ #(#else_body),* ] }
            } else {
                quote! { { #(#body),* } }
            };

            return quote! {
                if #condition {
                    either::Either::Left(#content)
                } else {
                    either::Either::Right(#else_content)
                }
            };
        }

        quote! {
            if #condition {
                Some(#content)
            } else {
                None
            }
        }
    }
}
