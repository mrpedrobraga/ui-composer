#![allow(unused)]

use std::any::TypeId;

struct Rect {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

struct Color {
    r: u8,
    g: u8,
    b: u8,
}

struct Quad(Rect, Color);
struct Text(Rect, String, Color);

struct DrawTarget {}

trait DrawingMethod {}
struct QuadDrawing {}
impl QuadDrawing {
    #[inline(always)]
    fn poke(&self) {}
}
struct TextDrawing {}
impl TextDrawing {
    #[inline(always)]
    fn print<S: AsRef<str>>(&self, text: S) {
        println!("{}", text.as_ref());
    }
}
impl DrawingMethod for QuadDrawing {}
impl DrawingMethod for TextDrawing {}

trait Drawable {
    fn draw<R>(&self, target: &mut DrawTarget, resources: &R) where R: DrawingMethodProvider;
}
impl Drawable for Quad {
    fn draw<R>(&self, target: &mut DrawTarget, resources: &R) where R: DrawingMethodProvider {
        if let Some(method) = resources.get::<QuadDrawing>() {
            method.poke();
        } else {
            print!("Failed to draw quad!");
        }
    }
}
impl Drawable for Text {
    fn draw<R>(&self, target: &mut DrawTarget, resources: &R) where R: DrawingMethodProvider {
        if let Some(method) = resources.get::<TextDrawing>() {
            method.print(self.1.as_str());
        } else {
            print!("Failed to draw text!")
        }
    }
}
impl<A, B> Drawable for (A, B) where A: Drawable, B: Drawable {    
    fn draw<R>(&self, target: &mut DrawTarget, resources: &R) where R: DrawingMethodProvider {
        self.0.draw(target, resources);
        self.1.draw(target, resources);
    }
}

trait DrawingMethodProvider {
    #[inline(always)]
    fn get<R>(&self) -> Option<&R> where R: DrawingMethod + 'static;
}


impl<A, B> DrawingMethodProvider for (A, B) where A: DrawingMethod + 'static, B: DrawingMethod + 'static {
    #[inline(always)]
    fn get<R>(&self) -> Option<&R> where R: DrawingMethod + 'static {
        use std::any::Any;

        if TypeId::of::<A>() == TypeId::of::<R>() {
            return Some(unsafe { &*(&raw const self.0 as *const R) });
        }
        if TypeId::of::<B>() == TypeId::of::<R>() {
            return Some(unsafe { &*(&raw const self.1 as *const R) });
        }
        return None;
    }
}

pub fn main() {
    let resources = (QuadDrawing {}, TextDrawing {});
    let nodes = (
        Quad(Rect { x: 0, y: 0, w: 32, h: 32 }, Color { r: 0xff, g: 0xff, b: 0x00 }),
        Text(Rect { x: 0, y: 0, w: 32, h: 32 }, format!("Hello, there!"), Color { r: 0xff, g: 0xff, b: 0x00 }),
        //Text(Rect { x: 0, y: 0, w: 32, h: 32 }, format!("Welcome!"), Color { r: 0xff, g: 0xff, b: 0x00 }),
    );
    let mut target = DrawTarget {};
    nodes.draw(&mut target, &resources);
}
