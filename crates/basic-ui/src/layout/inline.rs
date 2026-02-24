use {
    ui_composer_core::prelude::LayoutItem,
    vek::{Extent2, Vec2},
};

type Offset = u32;

#[derive(Default)]
pub struct InlineContext {
    pub offset: Vec2<Offset>,
    pub container_size: Extent2<Offset>,
    pub max_line_height: Offset,
    pub inline_gap: Offset,
    pub cross_axis_gap: Offset,
}

impl InlineContext {
    pub fn new_line(&mut self) {
        self.offset.y += self.max_line_height + self.cross_axis_gap;
        self.offset.x = 0;
        self.max_line_height = 1;
    }

    pub fn prepare_for_next_item(&mut self) {
        self.offset.x += self.inline_gap;
    }
}

pub trait InlineItem {
    type Output;
    fn allocate(&mut self, cx: &mut InlineContext) -> Self::Output;
}

pub struct Inline<T>(pub T);

impl<T: LayoutItem> InlineItem for Inline<T> {
    type Output = Vec2<Offset>;

    fn allocate(&mut self, cx: &mut InlineContext) -> Self::Output {
        let size = self.0.get_natural_size();
        let (w, h) = (size.w as Offset, size.h as Offset);

        if cx.offset.x > 0 && cx.offset.x + w > cx.container_size.w {
            cx.new_line();
        }

        cx.max_line_height = cx.max_line_height.max(h);
        let pos = cx.offset;
        cx.offset.x += w;
        pos
    }
}

pub struct MonospaceText(pub String);

impl InlineItem for MonospaceText {
    type Output = Vec<Vec2<Offset>>;

    fn allocate(&mut self, cx: &mut InlineContext) -> Self::Output {
        let mut points = Vec::new();
        let words: Vec<&str> = self.0.split_whitespace().collect();

        for (i, word) in words.iter().enumerate() {
            let mut word_str = *word;
            while !word_str.is_empty() {
                let len = word_str.chars().count() as Offset;
                if cx.offset.x > 0 && cx.offset.x + len > cx.container_size.w {
                    cx.new_line();
                }

                let take = if len > cx.container_size.w {
                    cx.container_size.w.saturating_sub(cx.offset.x).max(1)
                } else {
                    len
                };

                points.push(cx.offset);
                cx.max_line_height = cx.max_line_height.max(1);
                cx.offset.x += take;

                let mid = word_str
                    .char_indices()
                    .map(|(i, _)| i)
                    .nth(take as usize)
                    .unwrap_or(word_str.len());
                word_str = &word_str[mid..];

                if !word_str.is_empty() {
                    cx.new_line();
                }
            }

            if i < words.len() - 1 {
                if cx.offset.x >= cx.container_size.w {
                    cx.new_line();
                } else {
                    cx.offset.x += 1;
                }
            }
        }
        points
    }
}

#[test]
fn test_inline_alloc() {
    use ui_composer_core::app::composition::layout::{ItemBox, Resizable};

    const W: usize = 10;
    const H: usize = 20;

    let mut cx = InlineContext {
        container_size: Extent2::new(W as Offset, H as Offset),
        max_line_height: 1,
        inline_gap: 1,
        cross_axis_gap: 1,
        ..Default::default()
    };

    let text_1 = "Once upon a time there was a";
    let box_size = Extent2::new(4, 2);
    let text_2 = "so beautiful princess that nobody knew.";

    let mut t1 = MonospaceText(text_1.into());
    let mut b = Inline(ItemBox::new(|_| {}).with_minimum_size(box_size.as_()));
    let mut t2 = MonospaceText(text_2.into());

    let t1_pts = t1.allocate(&mut cx);
    cx.prepare_for_next_item();
    let b_pt = b.allocate(&mut cx);
    cx.prepare_for_next_item();
    let t2_pts = t2.allocate(&mut cx);

    let mut buf = [[' '; W]; H];
    let render_text =
        |t: &str, pts: &[Vec2<Offset>], buf: &mut [[char; W]; H]| {
            for (word, pos) in t.split_whitespace().zip(pts) {
                for (i, c) in word.chars().enumerate() {
                    let (x, y) = (pos.x as usize + i, pos.y as usize);
                    if x < W && y < H {
                        buf[y][x] = c;
                    }
                }
            }
        };

    render_text(text_1, &t1_pts, &mut buf);
    for dy in 0..box_size.h as usize {
        for dx in 0..box_size.w as usize {
            let (x, y) = (b_pt.x as usize + dx, b_pt.y as usize + dy);
            if x < W && y < H {
                buf[y][x] = '█';
            }
        }
    }
    render_text(text_2, &t2_pts, &mut buf);

    println!("┌{}┐", "─".repeat(W));
    for row in buf {
        println!("│{}│", row.iter().collect::<String>());
    }
    println!("└{}┘", "─".repeat(W));
}
