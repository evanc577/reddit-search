use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{Event, HtmlInputElement, HtmlSelectElement, InputEvent};

pub mod search_box;
pub mod search_button;
pub mod select;
pub mod text_input;

fn input_value(e: InputEvent) -> String {
    let event: Event = e.dyn_into().unwrap_throw();
    let event_target = event.target().unwrap_throw();
    let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
    target.value()
}

fn select_value(e: InputEvent) -> String {
    let event: Event = e.dyn_into().unwrap_throw();
    let event_target = event.target().unwrap_throw();
    let target: HtmlSelectElement = event_target.dyn_into().unwrap_throw();
    target.value()
}

#[derive(Clone, PartialEq)]
pub enum Width {
    Full,
    Half,
}

impl Width {
    pub fn class(&self) -> String {
        match self {
            Self::Full => String::from("search_full"),
            Self::Half => String::from("search_half"),
        }
    }
}
