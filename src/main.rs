use std::env;
use std::process;
use std::io::prelude::*;
  
extern crate url; 
extern crate reqwest;
extern crate select;

use url::percent_encoding::{utf8_percent_encode, DEFAULT_ENCODE_SET};
use colored::*;
use regex::Regex;
use select::document::Document;
use select::predicate::Attr;


fn main() {
	let args: Vec<String> = env::args().collect();
	let mut stderr = std::io::stderr();
    if args.len() < 2 {
        writeln!(&mut stderr, "Usage: explain [COMMAND] [ARG 1]..[ARG N]\nExample: explain ls -l -a\n").expect("couldn't write to stderr");
        process::exit(1);
    }
    let query = utf8_percent_encode(&args[1..].join(" "), DEFAULT_ENCODE_SET).to_string();
    explain(query);
}

fn explain(query: String) {
    let base_url = String::from("https://explainshell.com/explain?cmd=");
    let url = format!("{}{}", base_url, query);
    let resp = reqwest::get(&url).unwrap();
    assert!(resp.status().is_success());
    let delimiter = format!("\n\n{}", "_".repeat(50));
    let regex = Regex::new("^[ \t]*-.+").unwrap();
    Document::from_read(resp)
        .unwrap()
        .find(Attr("class", "help-box"))
        //.for_each(|x| println!("\n{}{}", x.text().replace("\n   ", "\n"), delimiter));
        .for_each(|node| {
            node.text().split("\n").collect::<Vec<_>>().iter().for_each(|line| {
                let mut output = line.to_string().white();
                 if regex.is_match(line) {
                    output = line.to_string().bold().red();
                }
                print!("\n{}", output);
            });
            println!("{}", delimiter)
        });
}