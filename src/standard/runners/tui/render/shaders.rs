use image::RgbaImage;
use std::sync::OnceLock;
use vek::{Rgba, Vec2, Vec3};

pub struct PixelShaderInput {
    pub uv: Vec2<f32>,
    pub pixelCoord: Vec2<u32>,
    pub time: f32,
}

pub fn funky(PixelShaderInput { uv, time, .. }: PixelShaderInput) -> Rgba<f32> {
    // Center the coordinates (-1.0 to 1.0) and adjust for typical terminal aspect ratio
    let p = uv * 2.0 - Vec2::new(1.0, 1.0);

    // Create some "wobble" using sine waves and time
    let mut color = Vec3::new(0.0, 0.0, 0.0);

    for i in 1..4 {
        let i_f = i as f32;
        let uv_wobble = Vec2::new(
            p.x + 0.7 / i_f * (i_f * p.y + time + 0.3 * i_f).sin(),
            p.y + 0.4 / i_f * (i_f * p.x + time + 0.5 * i_f).cos(),
        );

        // Combine sine waves to create the "plasma" feel
        let val = (uv_wobble.x + time).sin()
            + (uv_wobble.y + time * 0.5).cos()
            + (uv_wobble.x + uv_wobble.y + time).sin();

        // Map values to funky RGB channels
        color.x += (val * std::f32::consts::PI).cos();
        color.y += (val * std::f32::consts::PI + 2.0).cos();
        color.z += (val * std::f32::consts::PI + 4.0).cos();
    }

    // Normalize and return
    Rgba::new(
        (color.x * 0.5 + 0.5).clamp(0.0, 1.0),
        (color.y * 0.5 + 0.5).clamp(0.0, 1.0),
        (color.z * 0.5 + 0.5).clamp(0.0, 1.0),
        1.0,
    )
}

static IMAGE: OnceLock<RgbaImage> = OnceLock::new();

fn get_image() -> &'static RgbaImage {
    IMAGE.get_or_init(|| {
        image::open("assets/camera.jpg")
            .expect("Failed to load image")
            .to_rgba8()
    })
}

pub fn image(PixelShaderInput { uv, .. }: PixelShaderInput) -> Rgba<f32> {
    let img = get_image();
    let (w, h) = img.dimensions();

    let x = (uv.x * (w as f32 - 1.0)).clamp(0.0, w as f32 - 1.0) as u32;
    let y = (uv.y * (h as f32 - 1.0)).clamp(0.0, h as f32 - 1.0) as u32;

    let pixel = img.get_pixel(x, y);

    Rgba::new(
        pixel[0] as f32 / 255.0,
        pixel[1] as f32 / 255.0,
        pixel[2] as f32 / 255.0,
        pixel[3] as f32 / 255.0,
    )
}
