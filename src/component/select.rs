use web_sys::InputEvent;
use yew::prelude::*;

use super::{select_value, Width};

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub id: String,
    pub options: Vec<String>,
    pub selected: String,
    pub on_input: Callback<String>,
}

#[function_component(Select)]
pub fn select(props: &Props) -> Html {
    let Props { id, options, selected, on_input } = props.clone();

    let oninput = Callback::from(move |event: InputEvent| {
        on_input.emit(select_value(event));
    });

    let width = Width::Half;

    let options: Vec<_> = options.iter().map(|s| html! {
        <option value={s.clone()} selected={s == &selected}>{s.clone()}</option>
    }).collect();

    html! {
        <select class={width.class()} id={id} {oninput}>
            {options}
        </select>
    }
}
