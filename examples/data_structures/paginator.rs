pub trait Paginator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
    fn previous(&mut self) -> Option<Self::Item>;
}

pub struct VecPag<T> {
    pub items: Vec<T>,
    pub index: usize,
}

impl<T: Copy> Paginator for VecPag<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.items.get(self.index).map(|i| {
            self.index += 1;
            *i
        })
    }

    fn previous(&mut self) -> Option<T> {
        if self.index == 0 {
            return None;
        };

        self.items.get(self.index - 1).map(|i| {
            self.index -= 1;
            *i
        })
    }
}
