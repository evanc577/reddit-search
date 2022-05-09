use yew::prelude::*;
use super::text_input::TextInput;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub width: Width,
    pub id: String,
    pub label: String,
    pub value: String,
    pub on_change: Callback<String>,
}

#[derive(Clone, PartialEq)]
pub enum Width {
    Full,
    Half,
}

impl Width {
    fn class(&self) -> String {
        match self {
            Self::Full => String::from("search_full"),
            Self::Half => String::from("search_half"),
        }
    }
}

#[function_component(SearchBox)]
pub fn text_input(props: &Props) -> Html {
    let Props { width, id, label, value, on_change } = props.clone();

    html! {
        <div class={width.class()}>
            <div>
                <label for={id.clone()}>{label}</label>
                <TextInput id={id} on_change={on_change} value={value} />
            </div>
        </div>
    }
}
