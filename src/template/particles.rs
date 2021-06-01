use crate::model::particle;

markup::define! {
    Particle<'a>(particle: &'a particle::Particle) {
        @match &particle.content {
            particle::Content::Text(content) => { @ParticleText { content } }
            particle::Content::Code(content) => { @ParticleCode { content } }
        }
    }

    ParticleText<'a>(content: &'a particle::Text) {
        .{"particle-text"} {
            @content.text
        }
    }

    ParticleCode<'a>(content: &'a particle::Code) {
        .{"particle-code"} {
            pre {
                @content.code
            }
            .language {
                "Code snippet in "
                @content.language
            }
        }
    }
}
