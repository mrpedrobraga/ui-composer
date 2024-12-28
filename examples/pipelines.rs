use ui_composer::prelude::Graphic;

fn main() {}

trait Pipeline {}

struct GraphicsPipeline {}
impl Pipeline for GraphicsPipeline {}

trait Drawable {
    type Pipeline: Pipeline;

    fn prepare(&mut self, pipeline: &mut Self::Pipeline);
}

impl Drawable for Graphic {
    type Pipeline = GraphicsPipeline;

    fn prepare(&mut self, pipeline: &mut Self::Pipeline) {
        println!("Drawing to pipeline!");
    }
}
