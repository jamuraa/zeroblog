use chrono::{DateTime, Utc};

#[derive(Clone)]
pub struct Text {
    pub text: String,
}

#[derive(Clone)]
pub struct Code {
    pub code: String,
    pub language: String,
}

#[derive(Clone)]
pub enum Content {
    Text(Text),
    Code(Code),
}

#[derive(Clone)]
pub struct Particle {
    pub stamp: DateTime<Utc>,
    pub in_summary: bool,
    pub content: Content,
}
