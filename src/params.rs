use gloo_storage::{LocalStorage, Storage};

#[derive(Clone)]
pub struct SearchParams {
    pub subreddit: String,
    pub author: String,
    pub query: String,
    pub time_start: String,
    pub time_end: String,
}

impl SearchParams {
    pub fn load() -> Self {
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
            subreddit,
            author,
            query,
            time_start,
            time_end,
        }
    }

    pub fn store(&self) {
        LocalStorage::set("subreddit", self.subreddit.clone()).unwrap();
        LocalStorage::set("author", self.author.clone()).unwrap();
        LocalStorage::set("query", self.query.clone()).unwrap();
        LocalStorage::set("time_start", self.time_start.clone()).unwrap();
        LocalStorage::set("time_end", self.time_end.clone()).unwrap();
    }
}
