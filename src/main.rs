use std::fs;
use std::io::Read;
use std::process::exit;

mod arg_parser;
mod desktop_file_parser;

fn main() {
    let full_clargs: Vec<String> = std::env::args().collect();
    let clargs = &full_clargs[1..];

    let args = match arg_parser::Arguments::parse(clargs.to_vec()) {
        Ok(t) => t,
        Err(arg_parser::ParserError::InvalidArgument(t)) => {
            println!("Invalid Argument: {}", t);
            exit(1);
        }
        Err(arg_parser::ParserError::NoOutputFlag) => {
            println!("No output flag specified. Use --output <field> to specify the field to output for each application");
            exit(1);
        }
        Err(arg_parser::ParserError::NoArguments) => {
            println!("No arguments provided");
            exit(1);
        }
    };

    let entries = fs::read_dir("/usr/share/applications").unwrap();

    let mut data = desktop_file_parser::DesktopFile::init();

    let mut stdout = String::from("");

    if args.stdin.is_none() {
        for entry in entries {
            
            let filepath = entry.unwrap().path().into_os_string().into_string().unwrap();
            let file = fs::read_to_string(&filepath).expect("Failed to open");
            
            data.parse_desktop_file(file, filepath);
            if data.passes_checks(&args) {
                match data.get(&args.output.as_ref().expect("")) {
                    desktop_file_parser::GetValue::String(s) => stdout.push_str(s),
                    desktop_file_parser::GetValue::VecString(s) => {
                        for i in s {
                            stdout.push_str(i);
                            stdout.push_str(",");
                        }
                        let _ = stdout.pop();
                    }
                }
                stdout.push_str("\n");
            }
        }

    } else {
        let mut stdin = String::from("");
        let _ = std::io::stdin().read_to_string(&mut stdin);
        stdin.make_ascii_lowercase();
        stdin = stdin.trim().to_string();

        for entry in entries {
            let filepath = entry.unwrap().path().into_os_string().into_string().unwrap();
            let file = fs::read_to_string(&filepath).expect("Failed to open");
            data.parse_desktop_file(file, filepath);

            let app_matches_stdin = match data.get(args.stdin.as_ref().expect("")) {
                desktop_file_parser::GetValue::String(s) => s.to_lowercase() == stdin,
                desktop_file_parser::GetValue::VecString(s) => s.contains(&stdin),
            };

            if app_matches_stdin {
                match data.get(&args.output.as_ref().expect("")) {
                    desktop_file_parser::GetValue::String(s) => stdout.push_str(s),
                    desktop_file_parser::GetValue::VecString(s) => {
                        for i in s {
                            stdout.push_str(i);
                            stdout.push_str(",");
                        }
                        let _ = stdout.pop();
                    }
                }
                stdout.push_str("\n");
            }
        }
    }

    println!("{}", stdout);
}
