use web_sys::InputEvent;
use yew::prelude::*;

use super::input_value;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub id: String,
    pub value: String,
    pub on_change: Callback<String>,
}

#[function_component(TextInput)]
pub fn text_input(props: &Props) -> Html {
    let Props { id, value, on_change } = props.clone();

    let oninput = Callback::from(move |input_event: InputEvent| {
        on_change.emit(input_value(input_event));
    });

    html! {
        <input type="text" id={id} {value} {oninput} />
    }
}
