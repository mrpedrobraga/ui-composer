use super::runner::Own;
use crate::app::composition::elements::Element;
use crate::runners::tui::runner::TerminalEnvironment;
use futures_signals::signal::Signal;
use pin_project::pin_project;
use std::pin::Pin;
use std::task::{Context, Poll};

