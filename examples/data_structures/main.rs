use paginator::{Paginator, VecPag};
mod paginator;

fn main() {
    let mut o = VecPag {
        items: vec![1, 2, 3],
        index: 0,
    };
    dbg!(o.next());
    dbg!(o.next());
    dbg!(o.next());
    dbg!(o.next());
    dbg!(o.previous());
    dbg!(o.previous());
    dbg!(o.previous());
    dbg!(o.previous());
}
