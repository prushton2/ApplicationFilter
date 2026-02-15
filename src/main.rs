use std::env::args;
use std::fs;
use std::process::exit;

mod arg_parser;
mod desktop_file_parser;

fn main() {
    // let arg_string: String = String::from("--category network --type application --output name");
    // let vec: Vec<String> = arg_string.split(" ").map(|s| s.to_string()).collect();
    // ;
    let clargs: Vec<String> = std::env::args().collect();
    let args = &clargs[1..];
    
    let args = match arg_parser::Arguments::parse(args.to_vec()) {
        Ok(t) => t,
        Err(arg_parser::ParserError::InvalidArgument(t)) => {
            println!("Invalid Argument: {}", t);
            exit(1);
        }
    };

    println!("{:?}", args);

    let entries = fs::read_dir("/usr/share/applications").unwrap();
    // println!("{:?}", entries);

    let mut data = desktop_file_parser::DesktopFile::init();

    let mut stdout = String::from("");

    for entry in entries {
        let filepath = entry.unwrap().path();
        let file = fs::read_to_string(&filepath).expect("Failed to open");
        
        data.parse_desktop_file(file, filepath.into_os_string().into_string().unwrap());
        if data.passes_checks(&args) {
            // println!("{:?}", data.name)
            stdout.push_str(data.name.as_str());
            stdout.push_str("\n");
            stdout.push_str(data.filename.as_str());
            stdout.push_str("\n");
            stdout.push_str("\n");
        }
    }

    println!("{}", stdout);
    
}
