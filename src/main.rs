use std::fs;
use std::io::Read;
use std::process::exit;

mod arg_parser;
mod desktop_file_parser;

fn main() {
    // let arg_string: String = String::from("--category network --type application --output name");
    // let vec: Vec<String> = arg_string.split(" ").map(|s| s.to_string()).collect();
    // ;
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
    };

    let entries = fs::read_dir("/usr/share/applications").unwrap();
    // println!("{:?}", entries);

    let mut data = desktop_file_parser::DesktopFile::init();

    let mut stdout = String::from("");

    if args.stdin.is_none() {
        for entry in entries {
            // let filepathbuf: PathBuf =;
            
            let filepath = entry.unwrap().path().into_os_string().into_string().unwrap();
            let file = fs::read_to_string(&filepath).expect("Failed to open");
            
            data.parse_desktop_file(file, filepath);
            if data.passes_checks(&args) {
                stdout.push_str(data.get(&args.output.as_ref().expect("")));
                stdout.push_str("\n");
            }
        }
    } else {
        let mut stdin = String::from("");
        let _ = std::io::stdin().read_to_string(&mut stdin);
        stdin = stdin.trim().to_lowercase();

        for entry in entries {
            let filepath = entry.unwrap().path().into_os_string().into_string().unwrap();
            let file = fs::read_to_string(&filepath).expect("Failed to open");
            data.parse_desktop_file(file, filepath);

            if data.get(args.stdin.as_ref().expect("")).to_lowercase() == stdin {
                stdout.push_str(data.get(args.output.as_ref().expect("")))
            }

        }
    }

    println!("{}", stdout);
    
}
