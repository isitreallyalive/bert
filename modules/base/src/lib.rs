use bert::prelude::*;

pub struct Base;

impl Module for Base {
    fn new() -> Self {
        Self {}
    }

    fn name(&self) -> &str {
        "base"
    }
}

create_module!(Base);
