use web_sys::InputEvent;
use yew::prelude::*;

use super::{select_value, Width};

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub width: Width,
    pub id: String,
    pub class: String,
    pub label: String,
    pub options: Vec<String>,
    pub selected: String,
    pub on_input: Callback<String>,
}

#[function_component(Select)]
pub fn select(props: &Props) -> Html {
    let Props { width, id, class, label, options, selected, on_input } = props.clone();

    let oninput = Callback::from(move |event: InputEvent| {
        on_input.emit(select_value(event));
    });

    let options: Vec<_> = options.iter().map(|s| html! {
        <option value={s.clone()} selected={s == &selected}>{s.clone()}</option>
    }).collect();

    let class = format!("{} {}", width.class(), class);

    html! {
        <div {class}>
            <div>
                <label for={id.clone()} name={label.clone()}>{label}</label>
                <select id={id} {oninput}>
                    {options}
                </select>
            </div>
        </div>
    }
}
