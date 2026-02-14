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
                "name" => {self.name = parts.nth(0).expect("God i hope you dont see this error").to_string()}
                "exec" => {self.exec = parts.nth(0).expect("God i hope you dont see this error").to_string()}
                "type" => {self._type = parts.nth(0).expect("God i hope you dont see this error").to_string()}

                "categories" => {self.categories = parts.nth(0).expect("God i hope you dont see this error").to_string().split(";").map(|s| s.to_string()).collect()}
                "keywords" => {self.keywords = parts.nth(0).expect("God i hope you dont see this error").to_string().split(";").map(|s| s.to_string()).collect()}
                _ => {}
            }
        }
    }
}