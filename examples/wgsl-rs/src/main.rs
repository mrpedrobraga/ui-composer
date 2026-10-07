//! # wgsl_rs tests
//! 
//! Experimentation using [wgsl-rs](https://github.com/schell/wgsl-rs/tree/main).
//! 
//! Also check out [the book](https://renderling.xyz/wgsl-rs).

use ::wgsl_rs::linkage::wgpu::analyze_wgsl_module;
use wgsl_rs::wgsl;

fn main() {
    println!("{}", hello_triangle::WGSL_SOURCE.wgsl_source().unwrap());

    hello_triangle::FRAME.set(10);
    let _ = dbg!(hello_triangle::fragment());
    
    let linkage = analyze_wgsl_module(&hello_triangle::WGSL_SOURCE).unwrap();
    let _ = dbg!(linkage.fragment_entries);
}

#[wgsl]
pub mod hello_triangle {
    use wgsl_rs::std::*;

    uniform!(group(0), binding(0), FRAME: u32);

    #[vertex]
    pub fn vertex(#[builtin(vertex_index)] v_idx: u32) -> Vec4f {
        const POS: [Vec2f; 3] = [
            vec2f(0.0, 0.5),
            vec2f(-0.5, -0.5),
            vec2f(-0.5, 0.5)
        ];

        let position = POS[v_idx as usize];
        vec4f(position.x, position.y, 0.0, 1.0)
    }

    #[fragment]
    pub fn fragment() -> Vec4f {
        vec4f(1.0, sin(f32(get!(FRAME)) / 128.0), 0.0, 1.0)
    }
}