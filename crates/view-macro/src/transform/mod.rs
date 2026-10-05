use ::syn::punctuated::Punctuated;

use {
    proc_macro::TokenStream,
    proc_macro_error2::emit_error,
    quote::quote,
    syn::{Expr, Ident, Pat},
};
use {::syn::token};

pub mod codegen;
pub mod parser;

pub fn view_internal(input: TokenStream) -> TokenStream {
    let res: Result<NodeList, _> = syn::parse(input);
    match res {
        Ok(component) => component.to_tokens().into(),
        Err(e) => {
            emit_error!(e);
            quote!().into()
        }
    }
}

enum Node {
    // foo (bar = 1) { quz { ... } }
    Element(Element),

    // for x in y { ... }
    ForExpr(Box<ForExpr>),

    // if cond { ... }
    IfExpr(Box<IfExpr>),

    // { expression }
    Block(Expr),
}

struct NodeList(Vec<Node>);

struct Element {
    path: syn::Path,
    attributes: Vec<Attribute>,
    method_calls: Vec<MethodCall>,
    children_structure: ChildrenStructure,
    children: Vec<Node>,
}

struct Attribute {
    key: Ident,
    value: Option<Expr>,
}

struct MethodCall {
    dot: ::syn::Token![.],
    method: Ident,
    turbofish: Option<::syn::AngleBracketedGenericArguments>,
    paren: token::Paren,
    args: Punctuated<Expr, ::syn::Token![,]>
}

enum ChildrenStructure {
    IndividualArguments,
    ConsList,
}

struct ForExpr {
    pat: Pat,
    expr: Expr,
    body: Vec<Node>,
    empty_state: Option<Vec<Node>>,
}

struct IfExpr {
    condition: Expr,
    body: Vec<Node>,
    else_body: Option<Vec<Node>>,
}
