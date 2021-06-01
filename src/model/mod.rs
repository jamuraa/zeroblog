use chrono::prelude::*;

pub mod particle;

#[derive(Clone)]
pub struct PostVersion {
    pub title: String,
    pub particles: Vec<particle::Particle>,
    pub published: DateTime<Utc>,
    stamp: DateTime<Utc>,
}

impl PostVersion {
    pub fn particles_now(title: impl ToString, particles: Vec<particle::Particle>) -> Self {
        Self {
            title: title.to_string(),
            particles,
            published: Utc::now(),
            stamp: Utc::now(),
        }
    }

    /// A simple constructor that just makes a version that is published and stamped
    /// when it is made.
    fn make_now(title: impl ToString, body: impl ToString) -> Self {
        let body_particle = particle::Particle {
            stamp: Utc::now(),
            in_summary: true,
            content: particle::Content::Text(particle::Text {
                text: body.to_string(),
            }),
        };
        Self::particles_now(title, vec![body_particle])
    }

    fn make_rust(title: impl ToString, body: impl ToString, code: impl ToString) -> Self {
        let body_particle = particle::Particle {
            stamp: Utc::now(),
            in_summary: true,
            content: particle::Content::Text(particle::Text {
                text: body.to_string(),
            }),
        };
        let code_particle = particle::Particle {
            stamp: Utc::now(),
            in_summary: true,
            content: particle::Content::Code(particle::Code {
                code: code.to_string(),
                language: "Rust".to_owned(),
            }),
        };
        Self::particles_now(title, vec![body_particle, code_particle])
    }

    pub fn summary<'a>(&'a self) -> impl Iterator<Item = &'a particle::Particle> {
        self.particles.iter().filter(|p| p.in_summary)
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

    pub fn code(title: impl ToString, body: impl ToString, code: impl ToString) -> Self {
        let current = PostVersion::make_rust(title, body, code);
        Self { current }
    }

    pub fn current<'a>(&'a self) -> &'a PostVersion {
        &self.current
    }
}
