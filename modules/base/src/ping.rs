use bert::prelude::*;

pub struct Ping;

impl Command for Ping {
    fn name(&self) -> &str {
        "ping"
    }
}
