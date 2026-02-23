use crate::transform::ChildrenStructure;

use super::{Attribute, Element, ForExpr, ViewNode};
use proc_macro_error2::emit_error;
use syn::{
    braced, bracketed, parenthesized,
    parse::{self, Parse, ParseStream},
    Expr, Ident, Pat, Path, Token,
};

impl Parse for ViewNode {
    fn parse(input: ParseStream) -> parse::Result<Self> {
        if input.peek(Token![for]) {
            Ok(ViewNode::ForExpr(input.parse()?))
        } else if input.peek(syn::token::Brace) {
            let content;
            braced!(content in input);
            Ok(ViewNode::Block(content.parse()?))
        } else {
            Ok(ViewNode::Element(input.parse()?))
        }
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
                if content.peek(Token![,]) {
                    content.parse::<Token![,]>()?;
                }
            }
        }

        let mut children = Vec::new();
        let mut children_structure = ChildrenStructure::IndividualArguments;

        let lookahead = input.lookahead1();
        if lookahead.peek(syn::token::Brace) {
            let content;
            braced!(content in input);
            while !content.is_empty() {
                children.push(content.parse()?);
            }
        } else if lookahead.peek(syn::token::Bracket) {
            children_structure = ChildrenStructure::ConsList;
            let content;
            bracketed!(content in input);
            while !content.is_empty() {
                children.push(content.parse()?);
            }
        } else if lookahead.peek(syn::Ident)
            || lookahead.peek(Token![::])
            || lookahead.peek(Token![for])
        {
            children.push(input.parse()?);
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

impl Parse for ForExpr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![for]>()?;
        let pat =
            Pat::parse_multi_with_leading_vert(input).unwrap_or_else(|e| {
                emit_error!(e);
                syn::parse_quote!(_)
            });
        if input.parse::<Token![in]>().is_err() {
            emit_error!(input.span(), "expected `in`")
        };
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

        Ok(ForExpr { pat, expr, body })
    }
}

impl Parse for Attribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let key: Ident = input.parse()?;

        let value = if input.peek(Token![=]) {
            input.parse::<Token![=]>()?;
            Some(input.parse()?)
        } else {
            None
        };

        Ok(Attribute { key, value })
    }
}
