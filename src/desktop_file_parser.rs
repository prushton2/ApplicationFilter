use crate::arg_parser;

#[derive(Debug)]
pub struct DesktopFile {
    pub filename: String,
    pub name: String,
    pub exec: String,
    pub categories: Vec<String>,
    pub _type: String,
    pub keywords: Vec<String>,
}

impl DesktopFile {
    pub fn init() -> Self {
        return DesktopFile {
            filename: String::from(""),
            name: String::from(""),
            exec: String::from(""),
            categories: vec![],
            _type: String::from(""),
            keywords: vec![]
        }
    }

    pub fn parse_desktop_file(&mut self, file: String, name: String) {
        self.filename = name;
        for i in file.split("\n") {
            if i.chars().nth(0) == Some('[') && i != "[Desktop Entry]" {
                break;
            }

            let mut parts = i.split("=");

            // this is a lot, should improve
            match parts.nth(0).expect("No component").to_lowercase().as_str() {
                "name" => {self.name = parts.nth(0).expect("God i hope you dont see this error").to_string()}
                "exec" => {self.exec = parts.nth(0).expect("God i hope you dont see this error").to_string()}

                "type" => {self._type = parts.nth(0).expect("God i hope you dont see this error").to_string().to_lowercase()}

                "categories" => {self.categories = parts.nth(0).expect("God i hope you dont see this error").to_string().to_lowercase().split(";").map(|s| s.to_string()).collect()}
                "keywords" => {self.keywords = parts.nth(0).expect("God i hope you dont see this error").to_string().to_lowercase().split(";").map(|s| s.to_string()).collect()}
                _ => {}
            }
        }
    }

    pub fn passes_checks(&self, args: &arg_parser::Arguments) -> bool {
        for filter in &args.filters {
            if !self.passes_check(filter) {
                return false;
            }
        }
        return true;
    }

    fn passes_check(&self, filter: &arg_parser::Argument) -> bool {

        // i think i have a reason to learn macros now
        match filter {
            arg_parser::Argument::Category(t) => {
                for i in t.split(",").map(|s| s.to_string()) {
                    if !self.categories.contains(&i) {
                        return false
                    }
                }
                return true
            },
            arg_parser::Argument::Type(t) => {
                for i in t.split(",").map(|s| s.to_string()) {
                    if !self._type.contains(&i) {
                        return false
                    }
                }
                return true
            },
            arg_parser::Argument::Keywords(t) => {
                for i in t.split(",").map(|s| s.to_string()) {
                    if !self.keywords.contains(&i) {
                        return false
                    }
                }
                return true
            },
            arg_parser::Argument::Exclude(t) => return !self.passes_check(t),
            _ => return false
        }
    }

    pub fn get(&self, field: &str) -> &str {
        return match field {
            "name" => &self.name,
            "exec" => &self.exec,
            _ => &self.filename
        }
    }
}