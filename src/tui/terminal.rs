use {
    super::{
        backend::{Node, NodeDescriptor},
        pipeline::Render,
    },
    crate::{app::primitives::Event, prelude::LayoutItem},
    crossterm::{style::Stylize as _, ExecutableCommand as _},
    std::{io::Write as _, pin::Pin},
    vek::Vec2,
};

#[allow(non_snake_case)]
pub fn Terminal<A>(item: A) -> TerminalNodeDescriptor<A>
where
    A: LayoutItem,
    A::Content: Render,
{
    TerminalNodeDescriptor { item }
}

pub struct TerminalNodeDescriptor<N> {
    #[allow(unused)]
    item: N,
}

impl<N: LayoutItem + Send + Sync> NodeDescriptor for TerminalNodeDescriptor<N> {
    type Reified = TerminalNode<N>;

    fn reify(self) -> Self::Reified {
        TerminalNode { item: self.item }
    }
}

pub struct TerminalNode<N> {
    #[allow(unused)]
    item: N,
}

impl<N: LayoutItem + Send + Sync> Node for TerminalNode<N> {
    fn setup(&mut self) {
        // Nothing yet!
    }

    fn handle_event(&mut self, _event: Event) {
        // Nothing yet
    }

    fn draw(&self, stdout: &mut std::io::Stdout, rect: vek::Rect<u16, u16>) -> std::io::Result<()> {
        stdout.execute(crossterm::terminal::Clear(
            crossterm::terminal::ClearType::All,
        ))?;

        //self.item.draw(stdout, rect)?;
        let item_size = self.item.get_minimum_size();
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
                stdout.execute(crossterm::cursor::MoveTo(x, y))?.execute(
                    crossterm::style::PrintStyledContent(" ".on(crossterm::style::Color::Rgb {
                        r: color.r,
                        g: color.g,
                        b: color.b,
                    })),
                )?;
            }
        }

        stdout.flush()?;

        Ok(())
    }

    fn poll_processors(
        self: Pin<&mut Self>,
        _cx: &mut std::task::Context,
    ) -> std::task::Poll<Option<()>> {
        std::task::Poll::Ready(Some(()))
    }
}
