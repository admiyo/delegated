mod hosts;

extern crate getopts;
extern crate yaml_rust;
use linked_hash_map;

use getopts::Options;
use std::env;

use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;
use yaml_rust::Yaml;
use yaml_rust::YamlEmitter;
use yaml_rust::YamlLoader;
use linked_hash_map::LinkedHashMap;



fn add_host(map:  &&LinkedHashMap<yaml_rust::Yaml, yaml_rust::Yaml>,
            hosts: &mut hosts::Hosts){
    let adders = map.get( &Yaml::from_str("add")).unwrap().as_hash().unwrap();
    for (name, address) in adders {
        let n = name.as_str().unwrap();
        let a = address.as_str().unwrap();
        hosts.add(n, a);
    }
}                   


fn del_host(map:  &&LinkedHashMap<yaml_rust::Yaml, yaml_rust::Yaml>,
            hosts: &mut hosts::Hosts){
    let deletes = map.get( &Yaml::from_str("del")).unwrap().as_hash().unwrap();
    for (name, address) in deletes {
        let n = name.as_str().unwrap();
        let a = address.as_str().unwrap();
        hosts.del(n, a);
    }
}                   




// one possible implementation of walking a directory only visiting files
fn visit_dirs(dir: &Path, hosts: &mut hosts::Hosts) -> io::Result<()> {
    if dir.is_dir() {
        for e in fs::read_dir(dir)? {            
            let entry = e?;
            let path = entry.path();
            let s: String;
            match path.to_str(){
                Some(x) => s = x.to_string(),
                None => s = String::new()
            }
            if path.is_dir() {
                visit_dirs(&path, hosts)?;
            } else {
                if s.ends_with("yml"){
                    println!("{}",s);
                    let contents = fs::read_to_string(path)
                        .expect("Something went wrong reading the file");
                    let docs: std::vec::Vec<yaml_rust::Yaml> = YamlLoader::load_from_str(&contents).unwrap();
                    let map:  &&LinkedHashMap<yaml_rust::Yaml, yaml_rust::Yaml>  = &docs[0].as_hash().unwrap();
                    add_host(map, hosts);
                }
            }
        }
    }
    Ok(())
}

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
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let mut manager = hosts::Hosts::new(contents.to_string());

    let dir: String;
    match matches.opt_str("d"){
        Some(x) => dir = x,
        None => dir = "/var/lib/hosts".to_string()
    }

    println!("Directory = {}", dir);
    let dir_path = Path::new(&dir);


    visit_dirs(dir_path, &mut manager);
    
   print!("{}", manager.list());
}
