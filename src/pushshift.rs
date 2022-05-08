use serde::Deserialize;
use time::{format_description, OffsetDateTime, UtcOffset};
use yew::prelude::*;

#[derive(Deserialize, Debug, Clone)]
pub struct RedditPost {
    pub subreddit: String,
    pub author: String,
    #[serde(rename = "created_utc")]
    pub time: i64,
    pub body: String,
    pub permalink: String,
    #[serde(skip)]
    tz_offset: i64,
}

impl RedditPost {
    pub fn html(&self) -> Html {
        html! {
            <div class="reddit_post">
                <a href={self.permalink.clone()} target="_blank" rel="noopener noreferrer" title="View on Reddit">
                    <div class="post_header">
                        <div class="subreddit">{self.subreddit.clone()}</div>
                        <div class="author">{self.author.clone()}</div>
                        <div class="time">{format_timestamp(self.time, self.tz_offset)}</div>
                    </div>
                    <div class="post_body">{self.body.clone()}</div>
                </a>
            </div>
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
struct RedditPostMultiple {
    data: Vec<RedditPost>,
}

pub fn parse_pushshift(
    json: impl AsRef<str>,
    tz_offset: i64,
) -> Result<Vec<RedditPost>, serde_json::Error> {
    let posts: RedditPostMultiple = serde_json::from_str(json.as_ref())?;
    let mut posts = posts.data;
    for post in posts.iter_mut() {
        post.permalink = format!("https://www.reddit.com{}?context=10000", post.permalink);
        post.subreddit = format!("r/{}", post.subreddit);
        post.author = format!("u/{}", post.author);
        post.tz_offset = tz_offset;
    }
    Ok(posts)
}

fn format_timestamp(ts: i64, tz_offset: i64) -> String {
    let dt = OffsetDateTime::from_unix_timestamp(ts)
        .unwrap()
        .to_offset(UtcOffset::from_whole_seconds(tz_offset as i32 * 60).unwrap());
    let format =
        format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]").unwrap();
    dt.format(&format).unwrap()
}
