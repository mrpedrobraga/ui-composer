use crate::transform::{ChildrenStructure, NodeList};

use super::{Attribute, Element, ForExpr, IfExpr, Node};
use proc_macro_error2::emit_error;
use syn::{
    braced, bracketed, parenthesized,
    parse::{self, Parse, ParseStream},
    Expr, Ident, Pat, Path, Token,
};

impl Parse for Node {
    fn parse(input: ParseStream) -> parse::Result<Self> {
        if input.peek(Token![for]) {
            Ok(Node::ForExpr(input.parse()?))
        } else if input.peek(Token![if]) {
            Ok(Node::IfExpr(input.parse()?))
        } else if input.peek(syn::token::Brace) {
            let content;
            braced!(content in input);
            Ok(Node::Block(content.parse()?))
        } else if input.peek(syn::Ident) && input.peek2(syn::Token![!]) {
            Ok(Node::Block(syn::Expr::Macro(syn::ExprMacro {
                attrs: Vec::new(),
                mac: input.parse()?,
            })))
        } else if input.peek(syn::Ident) {
            Ok(Node::Element(input.parse()?))
        } else {
            Ok(Node::Block(input.parse()?))
        }
    }
}

impl Parse for NodeList {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut body = Vec::new();
        while !input.is_empty() {
            body.push(input.parse()?);
        }
        Ok(NodeList(body))
    }
}

impl Parse for Element {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let path: Path = input.parse()?;

        let mut attributes = Vec::new();
        if input.peek(syn::token::Paren) {
            let content;
            parenthesized!(content in input);
            while !content.is_empty() {
                attributes.push(content.parse()?);

                // TODO: Require a comma, unless it's the last item.
                if content.peek(Token![,]) {
                    content.parse::<Token![,]>()?;
                }
            }
        }

        let mut children = Vec::new();
        let mut children_structure = ChildrenStructure::IndividualArguments;

        let lookahead = input.lookahead1();
        /* Use {} to include several children in the view node. */
        if lookahead.peek(syn::token::Brace) {
            let content;
            braced!(content in input);
            while !content.is_empty() {
                children.push(content.parse()?);
            }
        /* Use [] to include several children on a viewnode with a variable number of children. */
        } else if lookahead.peek(syn::token::Bracket) {
            children_structure = ChildrenStructure::ConsList;
            let content;
            bracketed!(content in input);
            while !content.is_empty() {
                children.push(content.parse()?);
            }
        /* {} isn't necessary for a single child. */
        } else if lookahead.peek(syn::Ident)
            || lookahead.peek(Token![::])
            || lookahead.peek(Token![for])
            || lookahead.peek(Token![if])
        {
            children.push(input.parse()?);
        /* Even if the child is a string. */
        } else if lookahead.peek(syn::LitStr) {
            let lit_str: syn::LitStr = input.parse()?;
            children.push(Node::Block(syn::Expr::Lit(syn::ExprLit {
                attrs: Vec::new(),
                lit: syn::Lit::Str(lit_str),
            })));
        /* If something has no children you can use a comma instead of an empty {}. */
        } else if lookahead.peek(syn::token::Comma) {
            input.parse::<syn::token::Comma>()?;
        }

        Ok(Element {
            path,
            attributes,
            children_structure,
            children,
        })
    }
}

mod kw {
    syn::custom_keyword!(of);
}

impl Parse for ForExpr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![for]>()?;
        let pat =
            Pat::parse_multi_with_leading_vert(input).unwrap_or_else(|e| {
                emit_error!(e);
                syn::parse_quote!(_)
            });
        // if input.parse::<Token![in]>().is_err() {
        //     emit_error!(input.span(), "expected `in`")
        // };
        if input.parse::<kw::of>().is_err() {
            emit_error!(input.span(), "expected of")
        }

        // There's no `parse_without_eager_bracket` so we can't use square brackets.
        let expr = Expr::parse_without_eager_brace(input).unwrap_or_else(|e| {
            emit_error!(e);
            syn::parse_quote!(())
        });
        let content;
        braced!(content in input);
        let mut body = Vec::new();
        while !content.is_empty() {
            body.push(content.parse()?);
        }

        let empty_state = if input.peek(Token![else]) {
            input.parse::<Token![else]>()?;

            let else_content;
            braced!(else_content in input);
            let mut stmts = Vec::new();
            while !else_content.is_empty() {
                stmts.push(else_content.parse()?);
            }
            Some(stmts)
        } else {
            None
        };

        Ok(ForExpr {
            pat,
            expr,
            body,
            empty_state,
        })
    }
}

impl Parse for IfExpr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![if]>()?;

        // There's no `parse_without_eager_bracket` so we can't use square brackets.
        let condition =
            Expr::parse_without_eager_brace(input).unwrap_or_else(|e| {
                emit_error!(e);
                syn::parse_quote!(())
            });

        let content;
        braced!(content in input);
        let mut body = Vec::new();
        while !content.is_empty() {
            body.push(content.parse()?);
        }

        let else_body = if input.peek(Token![else]) {
            input.parse::<Token![else]>()?;

            let else_content;
            braced!(else_content in input);
            let mut stmts = Vec::new();
            while !else_content.is_empty() {
                stmts.push(else_content.parse()?);
            }
            Some(stmts)
        } else {
            None
        };

        Ok(IfExpr {
            condition,
            body,
            else_body,
        })
    }
}

impl Parse for Attribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let key: Ident = input.parse()?;

        let value = if input.peek(Token![:]) {
            input.parse::<Token![:]>()?;
            Some(input.parse()?)
        } else {
            None
        };

        Ok(Attribute { key, value })
    }
}
