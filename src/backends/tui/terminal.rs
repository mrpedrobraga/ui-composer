use crate::backends::tui::pipeline::Canvas;
use core::pin::Pin;
use core::task::{Context, Poll};
use vek::Rgba;
use {
    super::{
        backend::{Node, NodeRe},
        pipeline::RenderTui,
    },
    crate::prelude::{Event, LayoutItem},
    vek::Vec2,
};

#[allow(non_snake_case)]
pub fn Terminal<A>(item: A) -> TerminalNodeDescriptor<A>
where
    A: LayoutItem,
    A::Content: RenderTui,
{
    TerminalNodeDescriptor { item }
}

pub struct TerminalNodeDescriptor<N> {
    #[allow(unused)]
    item: N,
}

impl<N: LayoutItem + Send + Sync> Node for TerminalNodeDescriptor<N> {
    type Reified = TerminalNode<N>;

    fn reify(self) -> Self::Reified {
        TerminalNode { item: self.item }
    }
}

pub struct TerminalNode<N> {
    #[allow(unused)]
    item: N,
}

impl<N: LayoutItem + Send + Sync> NodeRe for TerminalNode<N> {
    fn setup(&mut self) {
        // Nothing yet!
    }

    fn handle_event(&mut self, _event: Event) {
        // Nothing yet
    }

    fn draw<C>(&self, canvas: &mut C, rect: vek::Rect<u16, u16>)
    where
        C: Canvas<Pixel = Rgba<u8>>,
    {
        // Clear canvas

        //self.item.draw(stdout, rect)?;
        #[allow(deprecated)]
        let item_size = self.item.get_natural_size();
        let texture_size = Into::<Vec2<f32>>::into(rect.extent().as_());
        let item_position = (texture_size - Into::<Vec2<f32>>::into(item_size)) / 2.0;

        let item_rect: vek::Aabr<u16> = vek::Rect::from((item_position, item_size)).as_().into();

        let color: vek::Rgba<u8> = vek::Rgba {
            r: 0x77,
            g: 0x3a,
            b: 0xf4,
            a: 0xff,
        };
        for y in item_rect.min.y..item_rect.max.y {
            for x in item_rect.min.x..item_rect.max.x {
                canvas.put_pixel(Vec2::new(x as u32, y as u32), color);
            }
        }

        // Flush canvas to framebuffer, possibly!
    }

    fn poll_processors(self: Pin<&mut Self>, _cx: &mut Context) -> Poll<Option<()>> {
        Poll::Ready(Some(()))
    }
}
