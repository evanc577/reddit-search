mod fetch;
mod params;
mod pushshift;
mod component;

use fetch::fetch;
use params::SearchParams;
use pushshift::{parse_pushshift, RedditComment};
use time::{format_description, PrimitiveDateTime, UtcOffset};
use url::Url;
use yew::prelude::*;

use component::search_box::{SearchBox, Width};

pub enum FetchState {
    NotFetching,
    Fetching,
    Success(Vec<RedditComment>, SearchType, SearchParams),
    Done,
    Failed(String),
}

enum Msg {
    Search,
    More,
    SetPsFetchState(FetchState),
    UpdateSubreddit(String),
    UpdateAuthor(String),
    UpdateQuery(String),
    UpdateTimeStart(String),
    UpdateTimeEnd(String),
}

struct Model {
    results: Vec<RedditComment>,
    state: FetchState,
    tz_offset: i64,
    params: SearchParams,
    // For use when "more-ing"
    last_params: Option<SearchParams>,
}

#[derive(Clone)]
pub enum SearchType {
    Initial,
    More,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        // Get current timezone offset
        let tz_offset = -js_sys::Date::new_0().get_timezone_offset() as i64;

        // Create model
        Self {
            results: Vec::new(),
            state: FetchState::NotFetching,
            tz_offset,
            params: SearchParams::load(),
            last_params: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Search => {
                self.results.clear();
                self.search(ctx, SearchType::Initial);
                false
            }
            Msg::More => {
                self.search(ctx, SearchType::More);
                false
            }
            Msg::UpdateSubreddit(s) => {
                self.params.subreddit = s;
                false
            }
            Msg::UpdateQuery(s) => {
                self.params.query = s;
                false
            }
            Msg::UpdateAuthor(s) => {
                self.params.author = s;
                false
            }
            Msg::UpdateTimeStart(s) => {
                log::info!("UpdateTimeStart {:?}", s);
                self.params.time_start = s;
                false
            }
            Msg::UpdateTimeEnd(s) => {
                log::info!("UpdateTimeEnd {:?}", s);
                self.params.time_end = s;
                false
            }
            Msg::SetPsFetchState(x) => {
                if let FetchState::Success(_, _, ref params) = x {
                    params.store();

                    // Update last search params
                    self.last_params = Some(params.clone());
                }

                match x {
                    FetchState::Success(r, SearchType::Initial, _) => {
                        self.results = r;
                        self.state = FetchState::Done;
                    }
                    FetchState::Success(mut r, SearchType::More, _) => {
                        self.results.append(&mut r);
                        self.state = FetchState::Done;
                    }
                    _ => self.state = x,
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // Search box
        let mut elems = vec![self.search_form(ctx)];

        // Results
        for r in &self.results {
            elems.push(r.html())
        }

        match &self.state {
            FetchState::Fetching => elems.push(html! { "Fetching..." }),
            FetchState::Failed(err) => elems.push(html! { err }),
            FetchState::Done => {
                // More button
                if !self.results.is_empty() {
                    elems.push(html! {
                        <button onclick={ctx.link().callback(|_| Msg::More)}>{"More"}</button>
                    });
                }
            }
            _ => (),
        }

        html! {
            <div>{ for elems.into_iter() }</div>
        }
    }
}

impl Model {
    fn search_form(&self, ctx: &Context<Self>) -> Html {
        let on_subreddit_change = ctx.link().callback(Msg::UpdateSubreddit);
        let on_author_change = ctx.link().callback(Msg::UpdateAuthor);
        let on_query_change = ctx.link().callback(Msg::UpdateQuery);
        let on_time_start_change = ctx.link().callback(Msg::UpdateTimeStart);
        let on_time_end_change = ctx.link().callback(Msg::UpdateTimeEnd);

        html! {
            <div class="search">
                <div>
                    <SearchBox width={Width::Half}
                        id={"subreddit"}
                        label={"Subreddit:"}
                        on_change={on_subreddit_change}
                        value={self.params.subreddit.clone()} />
                    <SearchBox width={Width::Half}
                        id={"author"}
                        label={"Author:"}
                        on_change={on_author_change}
                        value={self.params.author.clone()} />
                </div>

                <div>
                    <SearchBox width={Width::Half}
                        id={"time_start"}
                        label={"After:"}
                        on_change={on_time_start_change}
                        value={self.params.time_start.clone()} />
                    <SearchBox width={Width::Half}
                        id={"time_end"}
                        label={"Before:"}
                        on_change={on_time_end_change}
                        value={self.params.time_end.clone()} />
                </div>

                <div>
                    <SearchBox width={Width::Full}
                        id={"query"}
                        label={"Query:"}
                        on_change={on_query_change}
                        value={self.params.query.clone()} />
                </div>

                <button onclick={ctx.link().callback(|_| Msg::Search)} >
                { "Search" }
                </button>

                <script src={"bundle.js"}></script>
            </div>
        }
    }

    fn search(&mut self, ctx: &Context<Self>, search_type: SearchType) {
        static BASE_URL: &str = "https://api.pushshift.io/reddit/comment/search";
        let params = match search_type {
            SearchType::Initial => self.params.clone(),
            SearchType::More => self.last_params.clone().unwrap(),
        };

        let url = {
            let mut url = Url::parse(BASE_URL).unwrap();

            // Add GET query parameters
            url.query_pairs_mut()
                .append_pair("sort", "desc")
                .append_pair("limit", "10000")
                .append_pair("subreddit", &params.subreddit)
                .append_pair("author", &params.author)
                .append_pair("q", &params.query);

            if let Some(ts) = parse_time(&params.time_start, self.tz_offset) {
                url.query_pairs_mut().append_pair("after", &ts.to_string());
            }

            // If getting more posts/comments, add "before_id" GET parameter
            if let FetchState::Done = &self.state {
                if let Some(r) = self.results.last() {
                    url.query_pairs_mut()
                        .append_pair("before", &r.time.to_string());
                }
            } else if let Some(ts) = parse_time(&params.time_end, self.tz_offset) {
                url.query_pairs_mut().append_pair("before", &ts.to_string());
            }

            url.to_string()
        };

        // Message to send when search finishes
        {
            let tz_offset = self.tz_offset;
            ctx.link().send_future(async move {
                match fetch(url).await {
                    Ok(x) => match parse_pushshift(x, tz_offset) {
                        Ok(p) => Msg::SetPsFetchState(FetchState::Success(p, search_type, params)),
                        Err(e) => Msg::SetPsFetchState(FetchState::Failed(e.to_string())),
                    },
                    Err(e) => Msg::SetPsFetchState(FetchState::Failed(e.to_string())),
                }
            });
        }

        ctx.link()
            .send_message(Msg::SetPsFetchState(FetchState::Fetching));
    }
}

fn parse_time(s: impl AsRef<str>, offset: i64) -> Option<i64> {
    let format = format_description::parse("[year]-[month]-[day] [hour]:[minute]").unwrap();
    let ts = PrimitiveDateTime::parse(s.as_ref(), &format)
        .ok()?
        .assume_offset(UtcOffset::from_whole_seconds(60 * offset as i32).ok()?)
        .unix_timestamp();
    Some(ts)
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Model>();
}
