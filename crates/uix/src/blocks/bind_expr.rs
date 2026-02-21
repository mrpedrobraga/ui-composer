use rstml::recoverable::{ParseRecoverable, RecoverableContext};
use syn::parse::ParseStream;
use syn::{Expr, Pat, Token};
use crate::blocks::Block;

#[derive(Clone, Debug, syn_derive::ToTokens)]
pub struct BindExpr {
    pub keyword: Token![for],
    pub pat: Pat,
    pub token_in: Token![in],
    pub expr: Expr,
    pub block: Block,
}

impl ParseRecoverable for BindExpr {
    fn parse_recoverable(parser: &mut RecoverableContext, input: ParseStream) -> Option<Self> {
        let keyword = parser.parse_simple(input)?;
        let pat = parser.parse_mixed_fn(input, |_parse, input| {
            Pat::parse_multi_with_leading_vert(input)
        })?;
        let token_in = parser.parse_simple(input)?;
        let expr = parser.parse_mixed_fn(input, |_, input| {
            input.call(Expr::parse_without_eager_brace)
        })?;
        let block = parser.parse_recoverable(input)?;
        Some(BindExpr {
            keyword,
            pat,
            token_in,
            expr,
            block,
        })
    }
}