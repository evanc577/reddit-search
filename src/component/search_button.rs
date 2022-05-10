use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub state: SearchState,
    pub on_click: Callback<()>,
}

#[derive(Clone, PartialEq)]
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
            let onclick =  {
                let on_click = on_click.clone();
                Callback::from(move |_| {
                    on_click.emit(());
                })
            };
            let onkeypress = Callback::from(move |e: KeyboardEvent| {
                if e.code() == "Enter" {
                    on_click.emit(());
                }
            });

            html! {
                <div class="search_button button_active" tabindex={"0"} {onclick} {onkeypress}>
                    {text}
                </div>
            }
        }
        SearchState::Working(_) => {
            html! {
                <div class="search_button">
                    {text}
                </div>
            }
        }
    }
}
