use crate::template::layout::Layout;

markup::define! {
    NotFoundBody {
        h1 { "Page not found!" }
        p { "You look around a bit, but there definitely is nothing here." }
    }
}

pub(crate) const fn not_found() -> Layout<'static, NotFoundBody> {
    Layout {
        title: "Page not found",
        body: NotFoundBody {},
    }
}

pub(crate) fn internal(error: warp::reject::Rejection) -> impl std::fmt::Display {
    let formatted = format!("Maybe this will help: {:?}", error);
    Layout {
        title: "Unhandled Internal Error",
        body: markup::new! {
            h3 { "Should have thought of this.. " }
            p { @formatted }
        },
    }
}
