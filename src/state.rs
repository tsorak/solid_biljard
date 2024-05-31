#[derive(Debug, Clone)]
pub struct State {
    pub motd: String,
}

impl State {
    pub fn new(motd: &str) -> Self {
        Self {
            motd: motd.to_string(),
        }
    }
}
