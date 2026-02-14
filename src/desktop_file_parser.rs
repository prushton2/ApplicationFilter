use crate::arg_parser;

#[derive(Debug)]
pub struct DesktopFile {
    name: String,
    exec: String,
    categories: Vec<String>,
    _type: String,
    keywords: Vec<String>,
}

impl DesktopFile {
    pub fn init() -> Self {
        return DesktopFile {
            name: String::from(""),
            exec: String::from(""),
            categories: vec![],
            _type: String::from(""),
            keywords: vec![]
        }
    }

    pub fn parse_desktop_file(&mut self, file: String) {
        for i in file.split("\n") {
            let mut parts = i.split("=");
            // println!("{:?}", parts.nth(0));
            match parts.nth(0).expect("No component").to_lowercase().as_str() {
                "name" => {self.name = parts.nth(0).expect("God i hope you dont see this error").to_string().to_lowercase()}
                "exec" => {self.exec = parts.nth(0).expect("God i hope you dont see this error").to_string().to_lowercase()}
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
        return match filter {
            arg_parser::Argument::Category(t) => 
            arg_parser::Argument::Type(t) => 
            arg_parser::Argument::Keywords(t) => 
            arg_parser::Argument::Exclude(t) => !self.passes_check(t)
            _ => false
        }
    }
}