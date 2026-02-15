use crate::arg_parser;

#[derive(Debug)]
pub struct DesktopFile {
    pub file: String,
    pub name: String,
    pub exec: String,
    pub _type: String,
    pub nodisplay: bool,
    pub categories: Vec<String>,
    pub keywords: Vec<String>,
}

pub enum GetValue<'a> {
    String(&'a str),
    VecString(&'a Vec<String>),
    Bool(bool)
}

impl DesktopFile {
    pub fn init() -> Self {
        return DesktopFile {
            file: String::from(""),
            name: String::from(""),
            exec: String::from(""),
            nodisplay: false,
            _type: String::from(""),
            categories: vec![],
            keywords: vec![]
        }
    }

    pub fn parse_desktop_file(&mut self, file: String, filename: String) {
        self.file = filename;
        self.nodisplay = false;

        for i in file.split("\n") {
            if i.chars().nth(0) == Some('[') && i != "[Desktop Entry]" {
                break;
            }

            let mut parts = i.split("=");

            let lhs: String = match parts.nth(0) {
                Some(t) => t.to_lowercase().to_string(),
                None => continue
            };

            let mut rhs: String = match parts.nth(0) {
                Some(t) => t.to_string(),
                None => continue
            };

            // maybe theres something i can do about the splitting? i want to cut down on the memory allocations here.
            match lhs.as_str() {
                "name" => {
                    self.name = rhs;
                }
                "exec" => {
                    self.exec = rhs;
                }
                "type" => {
                    rhs.make_ascii_lowercase();
                    self._type = rhs;
                }
                "categories" => {
                    rhs.make_ascii_lowercase();
                    if rhs.chars().last() == Some(';') {
                        let _ = rhs.pop();
                    }
                    self.categories = rhs.split(";").map(|s| s.to_string()).collect()
                }
                "keywords" => {
                    if rhs.chars().last() == Some(';') {
                        let _ = rhs.pop();
                    }
                    rhs.make_ascii_lowercase();
                    self.keywords = rhs.split(";").map(|s| s.to_string()).collect()
                }
                "nodisplay" => {
                    rhs.make_ascii_lowercase();
                    self.nodisplay = rhs == "true";
                }
                _ => {}
            }
        }
    }

    pub fn passes_checks(&self, args: &arg_parser::Arguments) -> bool {
        for filter in &args.filters {
            if self.nodisplay && !args.display_nodisplay {
                return false;
            }
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

    pub fn get<'a>(&'a self, field: &str) -> GetValue<'a> {
        return match field {
            "name" => GetValue::String(&self.name),
            "exec" => GetValue::String(&self.exec),
            "type" => GetValue::String(&self._type),
            "categories" => GetValue::VecString(&self.categories),
            "keywords" => GetValue::VecString(&self.keywords),
            "nodisplay" => GetValue::Bool(self.nodisplay),
            "file" => GetValue::String(&self.file),
            _ => GetValue::String("")
        }
    }
}