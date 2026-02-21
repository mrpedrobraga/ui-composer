use rstml::node::{Node, NodeBlock, NodeElement, NodeFragment, NodeText, RawText};
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, quote_spanned, ToTokens};
use proc_macro::TokenStream;
use rstml::{Parser, ParserConfig};
use syn::spanned::Spanned;
use crate::blocks::closure_expr::ClosureExpr;
use crate::blocks::{CustomBlock, CustomExpr};
use crate::blocks::bind_expr::BindExpr;
use crate::config::is_container;

pub fn transform_tokens(input: TokenStream) -> TokenStream {
    let config: ParserConfig<CustomBlock> = ParserConfig::new()
        .recover_block(true)
        .number_of_top_level_nodes(1)
        .custom_node();
    let parser = Parser::new(config);
    let (nodes, errors) = parser.parse_recoverable(input).split_vec();

    let result = c_nodes(nodes.as_slice());
    let errors = errors.into_iter().map(|e| e.emit_as_expr_tokens());

    quote! {
        #(#errors;)*
        #result
    }
        .into()
}

fn c_nodes(nodes: &[Node<CustomBlock>]) -> TokenStream2 {
    let inner = nodes.iter().map(c_node);
    quote! {
        #(#inner),*
    }
}

fn c_node(node: &Node<CustomBlock>) -> TokenStream2 {
    match node {
        Node::Block(block) => c_block(block),
        Node::Text(text) => c_text(text),
        Node::Element(element) => c_element(element),
        Node::Fragment(fragment) => c_fragment(fragment),
        Node::Custom(custom_node) => c_custom(&custom_node.expression),
        Node::RawText(raw_text) => c_raw_text(raw_text),
        _ => quote!(),
    }
}

fn c_block(block: &NodeBlock) -> TokenStream2 {
    match block {
        NodeBlock::ValidBlock(block) => {
            let stmts = &block.stmts;
            match stmts.len() {
                0 => quote_spanned!(block.span() => ()),
                1 => quote_spanned!(block.span() => #(#stmts)*),
                _ => quote_spanned!(block.span() => { #(#stmts)* }),
            }
        }
        NodeBlock::Invalid(invalid) => invalid.to_token_stream(),
    }
}

fn c_text(text: &NodeText) -> TokenStream2 {
    let val = text.value_string();
    quote_spanned!(text.span() => #val)
}

fn c_raw_text(text: &RawText<CustomBlock>) -> TokenStream2 {
    let val = text.to_string_best();
    quote_spanned! {text.span() => #val}
}

fn c_element(element: &NodeElement<CustomBlock>) -> TokenStream2 {
    let open_tag_name = element.name();

    let attrs = element.attributes().iter().map(|attr| match attr {
        rstml::node::NodeAttribute::Attribute(a) => {
            let key = &a.key;
            let span = key.span();

            let method_name = quote::format_ident!("with_{}", key.to_string(), span = span);

            if let Some(value) = a.value() {
                quote!(.#method_name(#value))
            } else {
                quote!(.#method_name())
            }
        }
        _ => quote!(),
    });

    if element.close_tag.is_none() {
        return quote! {
            {
                #open_tag_name()
                #(#attrs)*
            }
        };
    }

    let children_iter = element.children.iter().map(c_node);
    let children_tokens = if is_container(&open_tag_name.to_string()) {
        quote_cons_list(children_iter)
    } else {
        quote!(#(#children_iter),*)
    };

    quote! {
        {
            #open_tag_name(#children_tokens)
            #(#attrs)*
        }
    }
}

fn c_fragment(f: &NodeFragment<CustomBlock>) -> TokenStream2 {
    let inner = f.children.iter().map(c_node);
    quote_cons_list(inner)
}

fn quote_cons_list(mut items: impl DoubleEndedIterator<Item = TokenStream2>) -> TokenStream2 {
    let last = items.next_back();

    match last {
        None => quote! { () },
        Some(last_item) => items.rfold(last_item, |acc, item| {
            quote! { (#item, #acc) }
        }),
    }
}

fn c_custom(e: &CustomExpr) -> TokenStream2 {
    match e {
        CustomExpr::Bind(bind_expr) => c_bind_expr(bind_expr),
        CustomExpr::Closure(closure_expr) => c_closure(closure_expr),
    }
}

fn c_bind_expr(e: &BindExpr) -> TokenStream2 {
    let pat = &e.pat;
    let expr = &e.expr;
    let block = &e.block;
    let block = c_nodes(block.body.as_slice());

    quote! {
        #expr.map(|#pat| #block)
    }
}

fn c_closure(e: &ClosureExpr) -> TokenStream2 {
    let ClosureExpr { move_token, or1_token, inputs, or2_token, block } = e;
    let block = c_node(block);

    quote::quote! {
        #move_token #or1_token #inputs #or2_token #block
    }
}