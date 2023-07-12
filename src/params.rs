use std::fmt::Display;
use std::str::FromStr;

use gloo_storage::{LocalStorage, Storage};

use crate::pushshift::{Reddit, RedditComment, RedditSubmission, RedditType};

#[derive(Clone, Debug)]
pub struct SearchParams {
    pub endpoint: Endpoint,
    pub subreddit: String,
    pub author: String,
    pub query: String,
    pub time_start: String,
    pub time_end: String,
}

impl SearchParams {
    pub fn load() -> Self {
        let endpoint =
            match LocalStorage::get("endpoint").map(|s: String| Endpoint::from_str(s.as_str())) {
                Ok(Ok(e)) => e,
                _ => Endpoint::Comment,
            };
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
        let time_start = match LocalStorage::get("time_start") {
            Ok(s) => s,
            Err(_) => String::new(),
        };
        let time_end = match LocalStorage::get("time_end") {
            Ok(s) => s,
            Err(_) => String::new(),
        };

        SearchParams {
            endpoint,
            subreddit,
            author,
            query,
            time_start,
            time_end,
        }
    }

    pub fn store(&self) {
        LocalStorage::set("endpoint", self.endpoint.clone().to_string()).unwrap();
        LocalStorage::set("subreddit", self.subreddit.clone()).unwrap();
        LocalStorage::set("author", self.author.clone()).unwrap();
        LocalStorage::set("query", self.query.clone()).unwrap();
        LocalStorage::set("time_start", self.time_start.clone()).unwrap();
        LocalStorage::set("time_end", self.time_end.clone()).unwrap();
    }
}

#[derive(Clone, Debug)]
pub enum Endpoint {
    Submission,
    Comment,
}

static SUBMISSION_STR: &str = "Submissions";
static COMMENT_STR: &str = "Comments";

impl Endpoint {
    pub fn url(&self) -> &'static str {
        match self {
            Self::Submission => "https://api.pullpush.io/reddit/submission/search",
            Self::Comment => "https://api.pullpush.io/reddit/comment/search",
        }
    }

    pub fn parse(
        &self,
        json: impl AsRef<str>,
        tz_offset: i64,
    ) -> Result<Vec<RedditType>, serde_json::Error> {
        match self {
            Self::Submission => RedditSubmission::parse_pushshift(json, tz_offset),
            Self::Comment => RedditComment::parse_pushshift(json, tz_offset),
        }
    }

    pub fn list() -> Vec<String> {
        vec![COMMENT_STR.into(), SUBMISSION_STR.into()]
    }
}

impl Display for Endpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Submission => write!(f, "{}", SUBMISSION_STR),
            Self::Comment => write!(f, "{}", COMMENT_STR),
        }
    }
}

impl FromStr for Endpoint {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            x if x == SUBMISSION_STR => Ok(Self::Submission),
            x if x == COMMENT_STR => Ok(Self::Comment),
            _ => Err(()),
        }
    }
}
