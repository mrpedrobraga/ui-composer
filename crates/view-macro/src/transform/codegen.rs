use {
    crate::transform::ViewNode,
    quote::{format_ident, quote},
};
use {
    crate::transform::{ChildrenStructure, Element},
    proc_macro2::TokenStream,
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
            ViewNode::ForExpr(for_expr) => {
                let pat = &for_expr.pat;
                let expr = &for_expr.expr;
                let body: Vec<_> =
                    for_expr.body.iter().map(|i| i.to_tokens()).collect();

                if body.len() > 1 {
                    quote! {
                        #expr.map(|#pat| list![ #(#body),* ])
                    }
                } else {
                    quote! {
                        #expr.map(|#pat| { #(#body),* })
                    }
                }
            }
        }
    }
}
