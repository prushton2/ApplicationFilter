use std::fs;

mod arg_parser;
mod desktop_file_parser;

fn main() {
    let arg_string: String = String::from("--category network --exclude --keywords gpu --output name");
    let vec: Vec<String> = arg_string.split(" ").map(|s| s.to_string()).collect();

    let args = arg_parser::Arguments::parse(vec).unwrap();

    println!("{:?}\n{:?}", arg_string, args);

    let mut entries = fs::read_dir("/usr/share/applications").unwrap();
    println!("{:?}", entries);

    let mut data = desktop_file_parser::DesktopFile::init();

    let mut stdout = String::from("");

    for entry in entries {
        let filepath = entry.unwrap().path();
        let file = fs::read_to_string(filepath).expect("Failed to open");
        
        data.parse_desktop_file(file);
        if data.passes_checks(&args) {
            
        }
    }

    
}
