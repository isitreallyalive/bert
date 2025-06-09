use bert::prelude::*;

pub struct Ping;

impl Command for Ping {
    fn name(&self) -> &str {
        "ping"
    }
}

create_module!(Base; cmd: Ping);
