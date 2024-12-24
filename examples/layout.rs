use ui_composer::prelude::functions::weighted_division_with_minima;

fn main() {
    let r = weighted_division_with_minima(100.0, &[2.0, 1.0], &[0.0, 0.0]);
    println!("{:?}", r);
}
