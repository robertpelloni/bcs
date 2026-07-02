use crate::core::global::bcsstring::BcsStringWrapper;
use std::collections::HashMap;

pub struct BcsCommandLineOption {
    pub name: BcsStringWrapper,
    pub description: BcsStringWrapper,
    pub value_name: BcsStringWrapper,
}

pub struct BcsCommandLineParser {
    options: HashMap<String, BcsCommandLineOption>,
    values: HashMap<String, BcsStringWrapper>,
}

impl BcsCommandLineParser {
    pub fn new() -> Self {
        BcsCommandLineParser {
            options: HashMap::new(),
            values: HashMap::new(),
        }
    }

    pub fn add_option(&mut self, opt: BcsCommandLineOption) {
        self.options.insert(opt.name.to_string(), opt);
    }

    pub fn process(&mut self, args: Vec<String>) {
        for arg in args {
            if arg.len() > 2 && arg.starts_with("--") {
                let flag = &arg[2..];
                if self.options.contains_key(flag) {
                    self.values.insert(flag.to_string(), BcsStringWrapper::new("true"));
                }
            }
        }
    }

    pub fn is_set(&self, name: &BcsStringWrapper) -> bool {
        self.values.contains_key(&name.to_string())
    }

    pub fn value(&self, name: &BcsStringWrapper) -> Option<&BcsStringWrapper> {
        self.values.get(&name.to_string())
    }

    pub fn show_help(&self) {
        println!("Usage:");
        for (name, opt) in &self.options {
            println!("  --{}\t{}", name, opt.description.to_string());
        }
        std::process::exit(0);
    }
}
