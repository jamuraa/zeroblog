use crate::model::{Post, PostVersion};
use crate::template::layout::Layout;
use crate::template::particles::Particle;

markup::define! {
    PostSummary<'a>(post: &'a PostVersion) {
        .post {
            .{"post-title"} { @post.title }
            .published {
                "Posted "
                @post.published.format("%Y %B %e, a %A at %l:%M%P").to_string()
             }
            .{"post-summary"} {
                @for particle in post.summary() {
                    @Particle { particle }
                }
            }
        }
    }

    PostList<'a>(posts: &'a [Post]) {
        .posts {
            @for post in *posts {
                @PostSummary { post: post.current() }
            }
        }
    }
}

pub fn blog<'a>(posts: &'a [Post]) -> Layout<'a, PostList> {
    Layout {
        title: "Marie's Blog",
        body: PostList { posts },
    }
}
