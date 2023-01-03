use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub state: SearchState,
    pub on_click: Option<Callback<()>>,
}

#[derive(Clone, PartialEq, Eq)]
pub enum SearchState {
    Idle(String),
    Working(String),
}

impl SearchState {
    fn text(&self) -> &str {
        match self {
            Self::Idle(s) => s,
            Self::Working(s) => s,
        }
    }
}

#[function_component(SearchButton)]
pub fn search_button(props: &Props) -> Html {
    let Props { state, on_click } = props.clone();
    let text = html! {
        <p>{state.text()}</p>
    };

    match state {
        SearchState::Idle(_) => {
            let onclick = on_click.map(|c| {
                let on_click = c.clone();
                Callback::from(move |_| {
                    on_click.emit(());
                })
            });

            html! {
                <button class="search_button button_active" {onclick}>
                    {text}
                </button>
            }
        }
        SearchState::Working(_) => {
            html! {
                <button class="search_button">
                    {text}
                </button>
            }
        }
    }
}
