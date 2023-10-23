use std::env;
use std::fs::File;
use std::io::BufReader;

use dot_properties::read_properties;

fn main() {
    let args: Vec<String> = env::args().collect();

    let [program_name, args @ ..] = &args[..] else {
        unreachable!("Who wouldn't give me a program name?!");
    };

    if args.is_empty() {
        eprintln!("Usage: {} <paths to .properties files...>", program_name);
        std::process::exit(1);
    }

    for props_path in args {
        println!("\nReading {:?}", &props_path);

        let file = match File::open(props_path) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("IO error: {}", e);
                continue;
            }
        };

        let mut reader = BufReader::new(file);
        match read_properties(&mut reader) {
            Ok(properties) => println!("{:#?}", properties),
            Err(e) => eprintln!("Parsing error: {}", e),
        }
    }
}
