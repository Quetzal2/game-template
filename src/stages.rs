use legion::prelude::Stage;
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Stages {
    Check,
    Update,
    Draw,
}

impl Stage for Stages {}

impl std::fmt::Display for Stages {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stages::Check => write!(f, "check"),
            Stages::Update => write!(f, "update"),
            Stages::Draw => write!(f, "draw"),
        }
    }
}