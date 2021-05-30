use chrono::prelude::*;

#[derive(Clone)]
pub struct PostVersion {
    pub title: String,
    pub body: String,
    pub published: DateTime<Utc>,
    stamp: DateTime<Utc>,
}

impl PostVersion {
    /// A simple constructor that just makes a version that is published and stamped
    /// when it is made.
    fn make_now(title: impl ToString, body: impl ToString) -> Self {
        Self {
            title: title.to_string(),
            body: body.to_string(),
            published: Utc::now(),
            stamp: Utc::now(),
        }
    }
}

#[derive(Clone)]
pub struct Post {
    current: PostVersion,
}

impl Post {
    pub fn new(title: impl ToString, body: impl ToString) -> Self {
        let current = PostVersion::make_now(title, body);
        Self { current }
    }

    pub fn current<'a>(&'a self) -> &'a PostVersion {
        &self.current
    }
}
