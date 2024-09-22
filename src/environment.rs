use std::collections::HashMap;
use std::env;

pub type Environment = HashMap<String, String>;

#[derive(Debug)]
pub struct EnvironmentBuilder {
    vars: Environment,
}

impl Default for EnvironmentBuilder {
    fn default() -> Self {
        Self {
            vars: HashMap::new(),
        }
    }
}

impl EnvironmentBuilder {
    pub fn build(self) -> Environment {
        self.vars
    }

    /// Set a key value pair
    pub fn set(mut self, key: &str, value: &str) -> Self {
        self.vars.insert(key.to_string(), value.to_string());
        self
    }

    /// Pull in all env vars from the current process
    pub fn with_process_env(mut self) -> Self {
        for (k, v) in env::vars() {
            self.vars.insert(k, v);
        }
        self
    }
}
