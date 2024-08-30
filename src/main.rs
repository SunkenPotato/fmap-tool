

use std::fs::{self};
use std::io::Read as _;

use std::path::Path;
use std::{collections::HashMap, fmt::Display, process::exit};

use clap::{arg, Arg, ArgGroup, Command};
use colored::Colorize;
use serde::{Serialize, Serializer};

#[derive(Serialize)]
struct HashMapWrapper {
    #[serde(serialize_with = "serialize_hashmap")]
    map: HashMap<char, u32>
}

#[allow(unused)]
fn serialize_hashmap<S>(map: &HashMap<char, u32>, serializer: S) -> Result<S::Ok, S::Error> 
where S: Serializer
{
    let string_map: HashMap<String, u32> = map
        .iter()
        .map(|(k,v)| (k.to_string(), *v))
        .collect();

    string_map.serialize(serializer)
}

impl Display for HashMapWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display_str = serde_json::to_string(&self.map).unwrap();

        write!(f, "{}", display_str)
    }
}

fn main() {
    let command = Command::new("fmap")
        .arg(arg!(-i --infile [file] "Input file (optional)"))
        .arg(arg!(-o --outfile [file] "Out file (optional) stdout is used otherwise"))
        .arg(arg!(-j --json "Print output as JSON").required(false)/*.requires("outfile")*/)
        .arg(arg!(--stdin "Use stdin as input"))
        .arg(arg!(-r --raw [string] "Use raw input"))
        .group(
            ArgGroup::new("inputs")
                .args(&["infile", "stdin", "raw"])
                .multiple(false)
                .required(true)
        )
        .get_matches();

    let mut input_string: String = String::new();
    let output_string: String;

    if let Some(raw_input) = command.get_one::<String>("raw") {
        input_string = raw_input.to_string();

    } else if let Some(input_file_path) = command.get_one::<String>("infile") {
        let read_file_result = match fs::read_to_string(input_file_path) {
            Ok(v) => v,
            Err(_) => {
                throw_error(format!("File {} not found!", input_file_path));
                return
            }
        };

        input_string = read_file_result;
    } else if command.get_flag("stdin") {
        let mut input = Vec::new();
        let stdin = std::io::stdin();
        let mut handle = stdin.lock();
        let _ = handle.read_to_end(&mut input);
        input_string = match String::from_utf8(input) {
            Ok(v) => v,
            Err(_) => {
                throw_error("An error occurred while reading from stdin".red());
                return;
            }
        };
    }

    let fmap = frequency_map(input_string).map;

    if command.get_flag("json") {
        output_string = match serde_json::to_string(&fmap) {
            Ok(v) => v,
            Err(_) => {
                throw_error("A JSON serialization error occurred.".red());
                return;
            }
        }
    } else {
        output_string = format!("{:#?}", fmap)
    }

    if let Some(v) = command.get_one::<String>("outfile") {
        let outfile_path = Path::new(v);

        if outfile_path.exists() {
            throw_error(format!("File {} is not empty, returning.", v).yellow())
        }

        match fs::write(outfile_path, output_string) {
            Ok(_) => println!("{}", format!("Wrote FMAP to {}", v).green().bold()),
            Err(_) => throw_error(format!("Could not write to file {}", v).red().bold())
        };
    } else {
        println!("{}", output_string);
    }

    
}

fn throw_error(err_string: impl Display) {
    throw_error0(err_string, 1);
}

fn throw_error0(err_string: impl Display, code: i32) {
    eprintln!("{}", err_string);
    exit(code);
}

fn frequency_map(string: String) -> HashMapWrapper {
    let mut fmap: HashMap<char, u32> = HashMap::new();

    for character in string.chars() {
            let opt_ctr = fmap.get(&character);
            if opt_ctr.is_some() {
                fmap.insert(character, opt_ctr.unwrap() + 1);
            } else {
                fmap.insert(character, 1);
            }
    };

    HashMapWrapper { map: fmap }
}