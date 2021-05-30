use crate::model::{Post, PostVersion};
use crate::template::layout::Layout;

markup::define! {
    Particle<'a>(post: &'a PostVersion) {
        .post {
            .{"post-title"} { @post.title }
            .published {
                "Posted "
                @post.published.format("%Y %B %e, a %A at %l:%M%P").to_string()
             }
            .{"post-body"} { @post.body }
        }
    }

    Particles<'a>(posts: &'a [Post]) {
        particles {
            @for post in *posts {
                @Particle { post: post.current() }
            }
        }
    }
}

pub fn blog<'a>(posts: &'a [Post]) -> Layout<'a, Particles> {
    Layout {
        title: "Marie's Blog",
        body: Particles { posts },
    }
}
