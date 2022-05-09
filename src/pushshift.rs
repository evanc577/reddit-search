use serde::Deserialize;
use time::{format_description, OffsetDateTime, UtcOffset};
use yew::prelude::*;

#[derive(Deserialize, Debug, Clone)]
pub struct RedditComment {
    pub subreddit: String,
    pub author: String,
    #[serde(rename = "created_utc")]
    pub time: i64,
    pub body: String,
    pub permalink: String,
    #[serde(skip)]
    tz_offset: i64,
    pub id: String,
}

impl RedditComment {
    pub fn html(&self) -> Html {
        html! {
            <div class="reddit_comment">
                <a href={self.permalink.clone()} target="_blank" rel="noopener noreferrer" title="View on Reddit">
                    <div class="comment_header">
                        <div class="subreddit">{self.subreddit.clone()}</div>
                        <div class="author">{self.author.clone()}</div>
                        <div class="time">{format_timestamp(self.time, self.tz_offset)}</div>
                    </div>
                    <div class="comment_body">{self.body.clone()}</div>
                </a>
            </div>
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
struct RedditCommentMultiple {
    data: Vec<RedditComment>,
}

pub fn parse_pushshift(
    json: impl AsRef<str>,
    tz_offset: i64,
) -> Result<Vec<RedditComment>, serde_json::Error> {
    let comments: RedditCommentMultiple = serde_json::from_str(json.as_ref())?;
    let mut comments = comments.data;
    for comment in comments.iter_mut() {
        comment.permalink = format!("https://www.reddit.com{}?context=10000", comment.permalink);
        comment.subreddit = format!("r/{}", comment.subreddit);
        comment.author = format!("u/{}", comment.author);
        comment.body = html_escape::decode_html_entities(&comment.body).into_owned();
        comment.tz_offset = tz_offset;
    }

    Ok(comments)
}

fn format_timestamp(ts: i64, tz_offset: i64) -> String {
    let dt = OffsetDateTime::from_unix_timestamp(ts)
        .unwrap()
        .to_offset(UtcOffset::from_whole_seconds(tz_offset as i32 * 60).unwrap());
    let format =
        format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]").unwrap();
    dt.format(&format).unwrap()
}
