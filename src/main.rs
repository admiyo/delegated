
mod hosts;

extern crate getopts;
use getopts::Options;
use std::env;
use std::fs;


fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    
    let mut opts = Options::new();
    opts.optopt("o", "operation", "operation to perform", "list, set, delete");
    opts.optflag("h", "help", "print this help menu");
    opts.optopt("f", "filename", "file to read and write","/etc/hosts");
    opts.optopt("a", "address", "ipaddress, either v4 or v6", "10.10.2.1");

    let args: Vec<String> = env::args().collect();
    
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") {
        let program = args[0].clone();
        print_usage(&program, opts);
        return;
    }    
    let operation: String;
    let filename: String;

    match matches.opt_str("o") {
        Some(x) => operation = x,
        None => operation = "list".to_string()
    }

    match matches.opt_str("f"){
        Some(x) => filename = x,
        None => filename = "./hosts".to_string()
    }
    
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    
    if operation == "list" {
        let wbyl: std::vec::Vec<std::vec::Vec<&str>> = hosts::words_by_line(&contents);
        hosts::list(wbyl);
    } else if operation == "set" {
        let wbyl: std::vec::Vec<std::vec::Vec<&str>> = hosts::words_by_line(&contents);

        let ipaddr = &args[3];
        let hostname = &args[4];
        hosts::set(wbyl, ipaddr, hostname);
    } else{
        println!("Unknown Operation");
    }
}

