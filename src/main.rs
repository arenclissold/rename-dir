use clap::{App, Arg};
use std::fs;
use std::path::{Path, PathBuf};

fn main() -> std::io::Result<()> {
    let matches = App::new("nazukeru")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("Renames directories and files in ascending order")
        .arg(
            Arg::with_name("start_number")
                .short('n')
                .long("start_number")
                .value_name("NUMBER")
                .help("Sets the start number for renaming")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("template")
                .short('t')
                .long("template")
                .value_name("TEMPLATE")
                .help("Sets the template for renaming")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let start_number = matches.value_of("start_number").unwrap().parse::<u32>().unwrap();
    let template = matches.value_of("template").unwrap();

    let current_dir = Path::new(".");

    let mut entries: Vec<_> = fs::read_dir(&current_dir)?.collect();
    entries.sort_by_key(|e| e.as_ref().unwrap().path().display().to_string());

    let mut number = start_number;
    for entry in entries {
        let entry = entry?;
        let new_path = create_new_path(&current_dir, template, number);
        fs::rename(entry.path(), &new_path)?;
        number += 1;
    }

    Ok(())
}

fn create_new_path(dir_path: &Path, template: &str, number: u32) -> PathBuf {
    let mut new_path = dir_path.to_path_buf();
    let number_string = format!("{:02}", number);
    let formatted_template = template.replace("{}", &number_string);
    new_path.push(formatted_template);
    new_path
}
