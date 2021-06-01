#![deny(warnings)]

use log::info;
use std::convert::Infallible;
use warp::{http::StatusCode, reply::html, Filter};

use crate::model::Post;

mod model;
mod template;

async fn render_errors(error: warp::reject::Rejection) -> Result<impl warp::Reply, Infallible> {
    let result = if error.is_not_found() {
        let html = html(template::error::not_found().to_string());
        warp::reply::with_status(html, StatusCode::NOT_FOUND)
    } else {
        let html = html(template::error::internal(error).to_string());
        warp::reply::with_status(html, StatusCode::INTERNAL_SERVER_ERROR)
    };
    Ok(result)
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let code_str = r#"
        let kim = 1;
        let stanley = "robinson";
        let mars = true;
    "#;

    let posts = vec![
        Post::new("Post one", "This is a post! I really think it is."),
        Post::code("Post two", "This post has some code in it.  yay!", code_str),
    ];

    // GET /
    let root = warp::path::end().map(move || html(template::blog::blog(&posts).to_string()));

    // GET /bye/:string
    let bye = warp::path("bye")
        .and(warp::path::param())
        .map(|name: String| format!("Good bye, {}!", name));

    let blog_routes = warp::get().and(root.or(bye));
    let log = warp::log("zeroblog");

    let routes = blog_routes.with(log).recover(render_errors);

    info!("Starting server...");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
