use std::fs;

mod arg_parser;
mod desktop_file_parser;

fn main() {
    let arg_string: String = String::from("--category network --exclude --keywords gpu --output name");
    let vec: Vec<String> = arg_string.split(" ").map(|s| s.to_string()).collect();

    let args = arg_parser::Argument::parse(vec);

    println!("{:?}\n{:?}", arg_string, args);

    let mut entries = fs::read_dir("/usr/share/applications").unwrap();
    println!("{:?}", entries);

    for entry in entries {
        println!("{:?}", entry);
    }

    
    // let file = fs::read_to_string("/usr/share/applications/mullvad-vpn.desktop").expect("Failed to open");
    // let mut data = desktop_file_parser::DesktopFile::init();
    // data.parse_desktop_file(file);
    // println!("{:?}", data);
}
