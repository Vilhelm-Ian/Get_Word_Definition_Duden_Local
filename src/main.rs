use regex::Regex;
use std::collections::{hash_map, HashMap};
use std::env;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::process::Command;
use std::str;

fn main() {
    let args: Vec<String> = env::args().collect();
    let word = Command::new("sh")
        .arg("-c")
        .arg(String::from("python ./__init__.py {:?}") + &args[1])
        .output()
        .expect("failed to execute process");
    let lemma = match str::from_utf8(&word.stdout) {
        Ok(v) => {
            let mut s = String::from(v);
            s.retain(|c| c != '\n' && c != '-' && c != '-' && c != '{' && c != '}');
            s
        }
        Err(e) => panic!("Invalid utf8 string {:?}", e),
    };
    let mut hash_map = HashMap::new();
    let file = match File::open(Path::new("../deu-deu_duden_universal_as_3_0.dsl")) {
        Ok(v) => v,
        Err(e) => panic!("problem opening file {:?}", e),
    };
    let r = BufReader::new(file);
    let s = utf16_reader::read_to_string(r);
    let mut temp_text = String::from("");
    let mut current_entry = "";
    for line in s.split('\n') {
        if line.contains('\t') {
            temp_text += line;
        } else {
            hash_map.insert(current_entry, temp_text.to_string());
            current_entry = line.trim_end();
            temp_text = String::from("");
        }
    }

    let re = Regex::new(r"\[i\](.*?)\[\/i\]").unwrap();
    println!("{:?}", lemma);
    let heute_def = match hash_map.get(lemma.as_str()) {
        Some(v) => v,
        None => panic!("Couldn't find lemma {:?}", lemma),
    };
    let mut output = String::from("");

    for result in re.find_iter(heute_def) {
        let re2 = Regex::new(r"\[(.*?)\]").unwrap();
        let mut definition = String::from(result.as_str());
        for result2 in re2.find_iter(&definition.clone()) {
            let replacement = result2.range().map(|_| " ").collect::<String>();
            definition.replace_range(result2.range(), &replacement);
        }
        definition = definition
            .split(' ')
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>()
            .join(" ");
        if definition.ends_with(':') {
            output += &definition;
            output += "\n";
        }
    }
    println!("{}", output);
}
