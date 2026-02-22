use crate::transform::ChildrenStructure;

use super::{Attribute, Element, ForExpr, ViewNode};
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

        if input.peek(syn::token::Brace) {
            let content;
            braced!(content in input);
            while !content.is_empty() {
                children.push(content.parse()?);
            }
        } else if input.peek(syn::token::Bracket) {
            children_structure = ChildrenStructure::ConsList;
            let content;
            bracketed!(content in input);
            while !content.is_empty() {
                children.push(content.parse()?);
            }
        } else if input.peek(syn::Ident)
            || input.peek(Token![::])
            || input.peek(Token![for])
        {
            children.push(input.parse()?);
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
        let pat = Pat::parse_multi_with_leading_vert(input)?;
        input.parse::<Token![in]>()?;
        let expr = Expr::parse_without_eager_brace(input)?;

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
