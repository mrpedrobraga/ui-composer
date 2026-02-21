use bind_expr::BindExpr;
use closure_expr::ClosureExpr;
use quote::{ToTokens, TokenStreamExt};
use rstml::{
    node::{CustomNode, Node},
    recoverable::{ParseRecoverable, RecoverableContext},
};
use syn::{
    parse::{Parse, ParseStream},
    token::Brace,
    Token,
};

pub mod bind_expr;
pub mod closure_expr;

#[derive(Clone, Debug, syn_derive::ToTokens)]
pub struct Block {
    #[syn(braced)]
    pub brace_token: Brace,
    #[syn(in = brace_token)]
    #[to_tokens(|tokens, val| tokens.append_all(val))]
    pub body: Vec<Node<CustomBlock>>,
}

impl ParseRecoverable for Block {
    fn parse_recoverable(parser: &mut RecoverableContext, input: ParseStream) -> Option<Self> {
        // we use this closure, because `braced!`
        // has private api and force its usage inside methods that return Result
        let inner_parser = |parser: &mut RecoverableContext, input: ParseStream| {
            let content;
            let brace_token = syn::braced!(content in input);
            let mut body = vec![];
            while !content.is_empty() {
                let Some(val) = parser.parse_recoverable(&content) else {
                    return Ok(None);
                };
                body.push(val);
            }
            Ok(Some(Block { brace_token, body }))
        };
        parser.parse_mixed_fn(input, inner_parser)?
    }
}

#[derive(Clone, Debug, syn_derive::ToTokens)]
pub enum CustomExpr {
    Bind(Box<BindExpr>),
    Closure(Box<ClosureExpr>),
}

impl ParseRecoverable for CustomExpr {
    fn parse_recoverable(parser: &mut RecoverableContext, input: ParseStream) -> Option<Self> {
        if input.peek(Token![for]) {
            parser
                .parse_recoverable(input)
                .map(Box::new)
                .map(CustomExpr::Bind)
        } else if input.peek(Token![move]) || input.peek(Token![|]) {
            parser
                .parse_recoverable(input)
                .map(Box::new)
                .map(CustomExpr::Closure)
        } else {
            None
        }
    }
}

#[derive(Clone, Debug, syn_derive::ToTokens)]
pub struct CustomBlock<T: ToTokens = Token![@]> {
    pub escape_token: T,
    pub expression: CustomExpr,
}

impl<T> ParseRecoverable for CustomBlock<T>
where
    T: ToTokens + Parse,
{
    fn parse_recoverable(parser: &mut RecoverableContext, input: ParseStream) -> Option<Self> {
        let escape_token = parser.parse_simple(input)?;
        let expression = parser.parse_recoverable(input)?;

        Some(Self {
            escape_token,
            expression,
        })
    }
}

impl<T> CustomNode for CustomBlock<T>
where
    T: ToTokens + Parse,
{
    fn peek_element(input: ParseStream) -> bool {
        let fork = input.fork();
        if fork.parse::<T>().is_err() {
            return false;
        }
        fork.peek(Token![for]) || fork.peek(Token![move]) || fork.peek(Token![|])
    }
}
