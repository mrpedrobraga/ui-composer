//! # Effects
//! 
//! UI Composer as a library takes a lot from functional programming languages.
//! Famously, FPs are said to have "no side effects."
//! 
//! This is a misrepresentation of them — they do have effects, but they are reified
//! into values which can be held and manipulated.
//! 
//! This module's main trait, [`ElementEffect`], represents such an effect.
//! 
//! For the purpose of UI, it represents the effects the items have on the computer.
//! For example, an "Image" value existing in the app causes the effect of rendering a picture,
//! an "Audio" value causes the effect of some sound playing, etc.

/// For handling the effects of state that will change later.
pub mod future;

/// For handling the effects of state that changes dynamically.
pub mod signal;

/// Represents the "effect" an element in the application has on its environment.
/// For example, an "Image" value existing in the app causes the effect of rendering a picture.
/// 
/// See the [module-level documentation][self].
#[diagnostic::on_unimplemented(
    message = "{Self} is not an effect applicable to {Env}."
)]
pub trait ElementEffect<Env> {}
