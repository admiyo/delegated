mod hosts;

extern crate getopts;
extern crate yaml_rust;

use getopts::Options;
use std::env;
use std::fs;
use std::path::Path;


fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optopt("f", "hostfile", "file to read and write","/etc/hosts");
    opts.optopt("d", "directory", "directory from which to read changes", "");

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
    let filename: String;
    match matches.opt_str("f"){
        Some(x) => filename = x,
        None => filename = "./hosts".to_string()
    }

    let dir: String;
    match matches.opt_str("d"){
        Some(x) => dir = x,
        None => dir = "/var/lib/hosts".to_string()
    }

    run(dir, filename);
    
}


fn run(dir: String, filename: String){
    println!("Directory = {}", dir);
    let dir_path = Path::new(&dir);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let mut manager = hosts::Hosts::new(contents.to_string());
    hosts::visit_dirs(dir_path, &mut manager).unwrap();
    print!("{}", manager.list());

}
