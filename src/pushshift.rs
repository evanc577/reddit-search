use serde::de::Error;
use serde::{Deserialize, Deserializer};
use time::{format_description, OffsetDateTime, UtcOffset};
use web_sys::HtmlImageElement;
use yew::prelude::*;

fn deserialize_decode_html<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    let decoded = html_escape::decode_html_entities(&s).into_owned();
    Ok(decoded)
}

fn deserialize_link_id<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    let id = s
        .split_once('_')
        .ok_or_else(|| anyhow::anyhow!("Invalid link_id {}", s))
        .map_err(D::Error::custom)?
        .1
        .to_owned();
    Ok(id)
}

pub trait Reddit {
    fn time(&self) -> i64;
    fn html(&self) -> Html;
    fn parse_pushshift(
        json: impl AsRef<str>,
        tz_offset: i64,
    ) -> Result<Vec<RedditType>, serde_json::Error>
    where
        Self: Sized;
    fn permalink(&self) -> String;
}

#[derive(Deserialize, Debug, Clone)]
struct RedditMultiple<T> {
    data: Vec<T>,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct RedditComment {
    subreddit: String,
    author: String,
    #[serde(rename = "created_utc")]
    time: i64,
    #[serde(deserialize_with = "deserialize_decode_html")]
    body: String,
    permalink: Option<String>,
    #[serde(skip)]
    tz_offset: i64,
    id: String,
    #[serde(deserialize_with = "deserialize_link_id")]
    link_id: String,
}

impl Reddit for RedditComment {
    fn time(&self) -> i64 {
        self.time
    }

    fn html(&self) -> Html {
        html! {
            <a class="reddit_comment" href={self.permalink()} target="_blank" rel="noopener noreferrer" title="View on Reddit">
                <div class="comment_header">
                    <div class="subreddit">{String::from("r/") + &self.subreddit}</div>
                    <div class="author">{String::from("u/") + &self.author}</div>
                    <div class="time">{format_timestamp(self.time, self.tz_offset)}</div>
                </div>
                <div class="comment_body">{self.body.clone()}</div>
            </a>
        }
    }

    fn parse_pushshift(
        json: impl AsRef<str>,
        tz_offset: i64,
    ) -> Result<Vec<RedditType>, serde_json::Error> {
        let comments: RedditMultiple<Self> = serde_json::from_str(json.as_ref())?;
        let mut comments = comments.data;
        for comment in comments.iter_mut() {
            comment.tz_offset = tz_offset;
        }

        let comments = comments.into_iter().map(RedditType::Comment).collect();
        Ok(comments)
    }

    fn permalink(&self) -> String {
        if let Some(l) = &self.permalink {
            format!("https://www.reddit.com{}?context=10000", l)
        } else {
            format!(
                "https://www.reddit.com/r/{}/comments/{}//{}?context=10000",
                self.subreddit, self.link_id, self.id
            )
        }
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct RedditSubmission {
    subreddit: String,
    author: String,
    #[serde(rename = "created_utc")]
    time: i64,
    permalink: Option<String>,
    #[serde(skip)]
    tz_offset: i64,
    id: String,
    is_self: bool,
    thumbnail: String,
    #[serde(deserialize_with = "deserialize_decode_html")]
    title: String,
    url: String,
    #[serde(deserialize_with = "deserialize_decode_html")]
    selftext: String,
}

impl Reddit for RedditSubmission {
    fn time(&self) -> i64 {
        self.time
    }

    fn html(&self) -> Html {
        let selftext = if self.is_self {
            html! {
                <div class="comment_body">{self.selftext.clone()}</div>
            }
        } else {
            html! {}
        };

        let thumbnail = if !self.is_self && self.thumbnail != "default" {
            let onerror = Callback::from(|e: Event| {
                if let Some(target) = e.target_dyn_into::<HtmlImageElement>() {
                    static BAD_IMAGE: &str = "bad-image.svg";
                    if !target.src().ends_with(BAD_IMAGE) {
                        target.set_src(BAD_IMAGE);
                    }
                }
            });
            html! {
                <img class="post_thumb"
                    alt="Reddit thumbnail"
                    src={self.thumbnail.clone()}
                    onerror={onerror} />
            }
        } else {
            html! {}
        };

        html! {
            <a class="reddit_comment" href={self.permalink()} target="_blank" rel="noopener noreferrer" title="View on Reddit">
                <div class="comment_header">
                    <div class="subreddit">{String::from("r/") + &self.subreddit}</div>
                    <div class="author">{String::from("u/") + &self.author}</div>
                    <div class="time">{format_timestamp(self.time, self.tz_offset)}</div>
                </div>
                <div class="post">
                    <div>
                        {thumbnail}
                    </div>
                    <div>
                        <div class="comment_title">{self.title.clone()}</div>
                        {selftext}
                    </div>
                </div>
            </a>
        }
    }

    fn parse_pushshift(
        json: impl AsRef<str>,
        tz_offset: i64,
    ) -> Result<Vec<RedditType>, serde_json::Error> {
        let submissions: RedditMultiple<Self> = serde_json::from_str(json.as_ref())?;
        let mut submissions = submissions.data;
        for submission in submissions.iter_mut() {
            submission.tz_offset = tz_offset;
        }

        let submissions = submissions
            .into_iter()
            .map(RedditType::Submission)
            .collect();
        Ok(submissions)
    }

    fn permalink(&self) -> String {
        if let Some(l) = &self.permalink {
            format!("https://www.reddit.com{}", l)
        } else {
            format!(
                "https://www.reddit.com/r/{}/comments/{}",
                self.subreddit, self.id
            )
        }
    }
}

#[derive(Debug)]
pub enum RedditType {
    Comment(RedditComment),
    Submission(RedditSubmission),
}

impl RedditType {
    pub fn time(&self) -> i64 {
        match self {
            Self::Comment(c) => c.time(),
            Self::Submission(s) => s.time(),
        }
    }

    pub fn html(&self) -> Html {
        match self {
            Self::Comment(c) => c.html(),
            Self::Submission(s) => s.html(),
        }
    }
}

fn format_timestamp(ts: i64, tz_offset: i64) -> String {
    let dt = OffsetDateTime::from_unix_timestamp(ts)
        .unwrap()
        .to_offset(UtcOffset::from_whole_seconds(tz_offset as i32 * 60).unwrap());
    let format =
        format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]").unwrap();
    dt.format(&format).unwrap()
}
