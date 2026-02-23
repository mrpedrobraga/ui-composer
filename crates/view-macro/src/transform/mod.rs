use {
    proc_macro::TokenStream,
    proc_macro_error2::emit_error,
    quote::quote,
    syn::{Expr, Ident, Pat},
};

pub mod codegen;
pub mod parser;

pub fn view_internal(input: TokenStream) -> TokenStream {
    let res: Result<ViewNode, _> = syn::parse(input);

    match res {
        Ok(component) => component.to_tokens().into(),
        Err(e) => {
            emit_error!(e);
            quote!().into()
        }
    }
}

enum ViewNode {
    // foo (bar = 1) { quz { ... } }
    Element(Element),
    // for x in y { ... }
    ForExpr(Box<ForExpr>),
    // { expression }
    Block(Expr),
}

struct Element {
    path: syn::Path,
    attributes: Vec<Attribute>,
    children_structure: ChildrenStructure,
    children: Vec<ViewNode>,
}

struct Attribute {
    key: Ident,
    value: Option<Expr>,
}

enum ChildrenStructure {
    IndividualArguments,
    ConsList,
}

struct ForExpr {
    pat: Pat,
    expr: Expr,
    body: Vec<ViewNode>,
}
