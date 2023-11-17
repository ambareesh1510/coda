use crate::env::Env;

struct Repl {
    env: Env,
}

impl Repl {
    fn new(s: String) -> Self {
        Self {
            env: Env::new(""),
        }
    }
}
