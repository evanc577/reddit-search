use serde::Deserialize;
use time::{format_description, OffsetDateTime, UtcOffset};
use web_sys::HtmlImageElement;
use yew::prelude::*;

pub trait Reddit {
    fn time(&self) -> i64;
    fn html(&self) -> Html;
    fn parse_pushshift(
        json: impl AsRef<str>,
        tz_offset: i64,
    ) -> Result<Vec<RedditType>, serde_json::Error>
    where
        Self: Sized;
}

#[derive(Deserialize, Debug, Clone)]
struct RedditMultiple<T> {
    data: Vec<T>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct RedditComment {
    subreddit: String,
    author: String,
    #[serde(rename = "created_utc")]
    time: i64,
    body: String,
    permalink: String,
    #[serde(skip)]
    tz_offset: i64,
    id: String,
}

impl Reddit for RedditComment {
    fn time(&self) -> i64 {
        self.time
    }

    fn html(&self) -> Html {
        html! {
            <a class="reddit_comment" href={self.permalink.clone()} target="_blank" rel="noopener noreferrer" title="View on Reddit">
                <div class="comment_header">
                    <div class="subreddit">{self.subreddit.clone()}</div>
                    <div class="author">{self.author.clone()}</div>
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
            comment.permalink =
                format!("https://www.reddit.com{}?context=10000", comment.permalink);
            comment.subreddit = format!("r/{}", comment.subreddit);
            comment.author = format!("u/{}", comment.author);
            comment.body = html_escape::decode_html_entities(&comment.body).into_owned();
            comment.tz_offset = tz_offset;
        }

        let comments = comments.into_iter().map(RedditType::Comment).collect();
        Ok(comments)
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct RedditSubmission {
    subreddit: String,
    author: String,
    #[serde(rename = "created_utc")]
    time: i64,
    permalink: String,
    #[serde(skip)]
    tz_offset: i64,
    id: String,
    is_self: bool,
    thumbnail: String,
    title: String,
    url: String,
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
            <a class="reddit_comment" href={self.permalink.clone()} target="_blank" rel="noopener noreferrer" title="View on Reddit">
                <div class="comment_header">
                    <div class="subreddit">{self.subreddit.clone()}</div>
                    <div class="author">{self.author.clone()}</div>
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
            submission.permalink = format!("https://www.reddit.com{}", submission.permalink);
            submission.subreddit = format!("r/{}", submission.subreddit);
            submission.author = format!("u/{}", submission.author);
            submission.title = html_escape::decode_html_entities(&submission.title).into_owned();
            submission.selftext =
                html_escape::decode_html_entities(&submission.selftext).into_owned();
            submission.tz_offset = tz_offset;
        }

        let submissions = submissions
            .into_iter()
            .map(RedditType::Submission)
            .collect();
        Ok(submissions)
    }
}

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
