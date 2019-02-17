mod hosts;

use std::env;
use std::fs;

fn words_by_line<'a>(s: &'a str) -> Vec<Vec<&'a str>> {
    s.lines().map(|line| {
        line.split_whitespace().collect()
    }).collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let _operation = &args[2];
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let wbyl: std::vec::Vec<std::vec::Vec<&str>> = words_by_line(&contents);

    if _operation == "list" {
        hosts::list(wbyl);
    } else if _operation == "set" {

        let ipaddr = &args[3];
        let hostname = &args[4];
        hosts::set(wbyl, ipaddr, hostname);
    } else{
        println!("Unknown Operation");
    }
}
