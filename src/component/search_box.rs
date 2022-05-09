use yew::prelude::*;

use super::text_input::TextInput;
use super::Width;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub width: Width,
    pub id: String,
    pub label: String,
    pub value: String,
    pub on_change: Callback<String>,
}

#[function_component(SearchBox)]
pub fn text_input(props: &Props) -> Html {
    let Props {
        width,
        id,
        label,
        value,
        on_change,
    } = props.clone();

    html! {
        <div class={width.class()}>
            <div>
                <label for={id.clone()}>{label}</label>
                <TextInput id={id} on_change={on_change} value={value} />
            </div>
        </div>
    }
}
