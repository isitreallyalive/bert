use bert::prelude::*;

mod ping;

pub struct Base;

impl Module for Base {
    fn new() -> Self {
        Self {}
    }

    fn name(&self) -> &str {
        "base"
    }

    fn commands(&self) -> Vec<Box<dyn Command>> {
        vec![Box::new(ping::Ping)]
    }
}

create_module!(Base);
