
use clap::{arg, command};

fn main() {
    let matches = command!()
        .arg(arg!(<subject> "The full string or file used  as your subject"))
        .arg(arg!(<pattern> "The string you want to replace"))
        .arg(arg!(<replacement> "The replacement"))
        .arg(arg!(-o --output <output> "optional output file").required(false))
        .get_matches();

    let subject = matches.get_one::<String>("subject").unwrap();
    let pattern = matches.get_one::<String>("pattern").unwrap();
    let replacement = matches.get_one::<String>("replacement").unwrap();
    let output = matches.get_one::<String>("output");

    let contents = match std::fs::read_to_string(subject) {
        Ok(contents) => contents.replace(pattern, replacement),
        Err(_error) =>  subject.replace(pattern, replacement)
    };

    match output {
        Some(output_path) => std::fs::write(output_path, contents).expect("No such file or directory"),
        None => println!("{}", contents)
    }

}
