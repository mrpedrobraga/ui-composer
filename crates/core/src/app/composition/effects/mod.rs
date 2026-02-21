pub mod future;
pub mod signal;

/// An effect that some element of a structure might produce.
///
/// For example, a `Graphic` might imply a rectangle should be drawn at some place on-screen.
/// Depending on the effect handler, this might result in quad instances being sent to the GPU
/// or rectangles drawn on the terminal or pixels in a GameBoy screen.
#[diagnostic::on_unimplemented(
    message = "{Self} is not an effect applicable to {Environment}."
)]
pub trait ElementEffect<Environment> {}
