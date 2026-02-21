use quote::TokenStreamExt;
use syn::{Pat, Token};
use syn::punctuated::Punctuated;
use rstml::node::Node;
use rstml::recoverable::{ParseRecoverable, RecoverableContext};
use syn::parse::ParseStream;
use crate::blocks::CustomBlock;

#[derive(Clone, Debug, syn_derive::ToTokens)]
pub struct ClosureExpr {
    pub move_token: Option<Token![move]>,
    pub or1_token: Token![|],
    #[to_tokens(|tokens, val| tokens.append_all(val))]
    pub inputs: Punctuated<Pat, Token![,]>,
    pub or2_token: Token![|],
    pub block: Box<Node<CustomBlock>>,
}

impl ParseRecoverable for ClosureExpr {
    fn parse_recoverable(parser: &mut RecoverableContext, input: ParseStream) -> Option<Self> {
        let header = parser.parse_mixed_fn(input, |_, i| {
            let move_token: Option<Token![move]> = i.parse()?;
            let or1_token: Token![|] = i.parse()?;

            let mut inputs = Punctuated::new();
            while !i.peek(Token![|]) {
                let arg = i.call(Pat::parse_single)?;
                inputs.push_value(arg);
                if i.peek(Token![|]) {
                    break;
                }
                let punct: Token![,] = i.parse()?;
                inputs.push_punct(punct);
            }

            let or2_token: Token![|] = i.parse()?;
            Ok((move_token, or1_token, inputs, or2_token))
        })?;

        let (move_token, or1_token, inputs, or2_token) = header;

        let block: Node<CustomBlock> = parser.parse_recoverable(input)?;
        let block = Box::new(block);

        Some(ClosureExpr {
            move_token,
            or1_token,
            inputs,
            or2_token,
            block,
        })
    }
}