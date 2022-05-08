mod fetch;
mod pushshift;
mod text_input;

use fetch::fetch;
use gloo_storage::{LocalStorage, Storage};
use pushshift::{parse_pushshift, RedditPost};
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::fetch::FetchError;
use crate::text_input::TextInput;

pub enum FetchState<T> {
    NotFetching,
    Fetching,
    Success(T),
    Failed(String),
}

enum Msg {
    GetPs,
    SetPsFetchState(FetchState<Vec<RedditPost>>),
    UpdateSubreddit(String),
    UpdateAuthor(String),
    UpdateQuery(String),
}

struct Model {
    ps_data: FetchState<Vec<RedditPost>>,
    subreddit: String,
    author: String,
    query: String,
    tz_offset: i64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        // Get current timezone offset
        let tz_offset = -js_sys::Date::new_0().get_timezone_offset() as i64;

        // Check storage and load previous search
        let subreddit = match LocalStorage::get("subreddit") {
            Ok(s) => s,
            Err(_) => String::new(),
        };
        let author = match LocalStorage::get("author") {
            Ok(s) => s,
            Err(_) => String::new(),
        };
        let query = match LocalStorage::get("query") {
            Ok(s) => s,
            Err(_) => String::new(),
        };

        // Create model
        Self {
            ps_data: FetchState::NotFetching,
            subreddit,
            author,
            query,
            tz_offset,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GetPs => {
                let url = format!(
                    "https://api.pushshift.io/reddit/comment/search?subreddit={}&author={}&q={}&limit=10000",
                    self.subreddit, self.author, self.query
                );
                let tz_offset = self.tz_offset;
                ctx.link().send_future(async move {
                    match fetch(url).await {
                        Ok(x) => match parse_pushshift(x, tz_offset) {
                            Ok(p) => Msg::SetPsFetchState(FetchState::Success(p)),
                            Err(e) => Msg::SetPsFetchState(FetchState::Failed(e.to_string())),
                        },
                        Err(e) => Msg::SetPsFetchState(FetchState::Failed(e.to_string())),
                    }
                });
                ctx.link()
                    .send_message(Msg::SetPsFetchState(FetchState::Fetching));
                false
            }
            Msg::UpdateSubreddit(s) => {
                self.subreddit = s;
                false
            }
            Msg::UpdateQuery(s) => {
                self.query = s;
                false
            }
            Msg::UpdateAuthor(s) => {
                self.author = s;
                false
            }
            Msg::SetPsFetchState(s) => {
                // Update storage
                if let FetchState::Success(_) = s {
                    LocalStorage::set("subreddit", self.subreddit.clone()).unwrap();
                    LocalStorage::set("author", self.author.clone()).unwrap();
                    LocalStorage::set("query", self.query.clone()).unwrap();
                }

                self.ps_data = s;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_subreddit_change = ctx.link().callback(Msg::UpdateSubreddit);
        let on_author_change = ctx.link().callback(Msg::UpdateAuthor);
        let on_query_change = ctx.link().callback(Msg::UpdateQuery);
        let mut elems = vec![
            html! {
                <>
                    <label for="subreddit">{ "Subreddit:" }</label>
                    <TextInput id={"subreddit"} on_change={on_subreddit_change} value={self.subreddit.clone()} />
                    <br/>

                    <label for="author">{ "Author:" }</label>
                    <TextInput id={"author"} on_change={on_author_change} value={self.author.clone()} />
                    <br/>

                    <label for="query">{ "Query:" }</label>
                    <TextInput id={"query"} on_change={on_query_change} value={self.query.clone()} />
                    <br />

                    <button onclick={ctx.link().callback(|_| Msg::GetPs)}>
                    { "Search" }
                </button>
                    </>
            },
            html! { <br /> },
        ];

        match &self.ps_data {
            FetchState::NotFetching => (),
            FetchState::Fetching => elems.push(html! { "Fetching..." }),
            FetchState::Success(posts) => {
                for post in posts {
                    elems.push(post.html())
                }
            }
            FetchState::Failed(err) => elems.push(html! { err }),
        }

        html! {
            <div>{ for elems.into_iter() }</div>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Model>();
}
