use std::io;
use std::fs;
use std::path::Path;
use yaml_rust::Yaml;
use yaml_rust::YamlLoader;
use linked_hash_map::LinkedHashMap;


pub struct Hosts {
    lines: Vec<Vec<String>>
}

impl Hosts{

    pub fn new (contents:  String) -> Hosts {
        let l = words_by_line(&contents);

        let mut lines = Vec::new();
        for i in l {
            let mut line: Vec<String> = Vec::new();
            for j in i {
                line.push(j.to_string());
            }
            lines.push(line);
        }
        Hosts{
            lines: lines
        }
    }

    pub fn list(& self) -> String{
        let mut out: String = String::new();
        for i in &self.lines {
            for j in i {
                out.push_str(&format!("{} ", j));
            }
            if out.ends_with(" "){
                out = out.trim().to_string();
            }
            out.push_str("\n");
        }
        out.clone()
    }


    pub fn add(& mut self, ipaddr:  &str, hostname:  &str){
        for i in &self.lines {
            if i.len() == 0{
                continue;
            }
            if i[0] == ipaddr{
                for j in i{
                    if j == &hostname{
                        //Found it.
                        return
                    }
                }
            }
        }
        self.lines.push(vec![ipaddr.to_string(), hostname.to_string()]);
    }

    pub fn del(& mut self, ipaddr:  &str, hostname:  &str){
        let host = vec![ipaddr.to_string(), hostname.to_string()];
        let i = self.lines.iter().position(|x| x == &host);
        match i {
            Some(l) => self.lines.remove(l),
            None => Vec::new(),
        };
    }

    
}

pub fn words_by_line<'a>(s: &'a str) -> Vec<Vec<&'a str>> {
    s.lines().map(|line| {
        line.split_whitespace().collect()
    }).collect()
}




fn add_host(map:  &&LinkedHashMap<yaml_rust::Yaml, yaml_rust::Yaml>,
            hosts: &mut Hosts){
    let adders = map.get( &Yaml::from_str("add")).unwrap().as_hash().unwrap();
    for (name, address) in adders {
        let n = name.as_str().unwrap();
        let a = address.as_str().unwrap();
        hosts.add(n, a);
    }
}                   


fn del_host(map:  &&LinkedHashMap<yaml_rust::Yaml, yaml_rust::Yaml>,
            hosts: &mut Hosts){
    let deletes = map.get( &Yaml::from_str("del")).unwrap().as_hash().unwrap();
    for (name, address) in deletes {
        let n = name.as_str().unwrap();
        let a = address.as_str().unwrap();
        hosts.del(n, a);
    }
}                   


// one possible implementation of walking a directory only visiting files
pub fn visit_dirs(dir: &Path, hosts: &mut Hosts) -> io::Result<()> {
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
                    del_host(map, hosts);
                }
            }
        }
    }
    Ok(())
}



pub fn run(dir: &str, filename: &str){
    let dir_path = Path::new(dir);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let mut hosts = Hosts::new(contents.to_string());
    visit_dirs(dir_path, &mut hosts).unwrap();
    print!("{}", hosts.list());
}



#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA_DIR: &str = "./data/hosts.d";
    const TEST_HOSTS_FILE: &str = "data/hosts";

    #[test]
    fn test_empty_list() {
        let  contents = "".to_string();
        let manager = Hosts::new(contents);
        let out = manager.list();
        assert_eq!(out,"");
    }
    #[test]
    fn test_sample_list() {
        let contents = SAMPLEDATA.to_string();
        let manager = Hosts::new(contents);
        let out = manager.list();
        assert_eq!(out,SAMPLEDATA);
    }


    #[test]
    fn test_delete() {
        let contents = "127.0.0.0 slashdot.org\n".to_string();
        let mut manager = Hosts::new(contents);
        let orig = manager.list();
        assert_eq!(orig, "127.0.0.0 slashdot.org\n");
        manager.del("127.0.0.0","slashdot.org");
        let out = manager.list();
        assert_eq!(out, "");
    }

    
    #[test]
    fn test_empty_add() {
        let contents = "".to_string();
        let mut manager = Hosts::new(contents);
        manager.add("127.0.0.0", "slashdot.org");
        let out = manager.list();
        assert_eq!(out,"127.0.0.0 slashdot.org\n");
    }


    #[test]
    fn test_functional() {
        run(TEST_DATA_DIR, TEST_HOSTS_FILE);
    }

    #[test]
    fn test_parse() {
        let contents = fs::read_to_string(TEST_HOSTS_FILE)
            .expect("Something went wrong reading the file");
        let hosts = Hosts::new(contents.to_string());
        hosts.list();
    }


    
    const SAMPLEDATA: &'static str =
"127.0.0.1 localhost localhost.localdomain localhost4 localhost4.localdomain4

10.0.0.112 ayoungP50
10.0.0.104 dialga.home.younglogic.net dialga
10.0.0.30 munchlax.home.younglogic.net munchlax
10.127.0.3 passimian passimian.home.younglogic.net
10.127.0.7 diglett diglett.home.younglogic.net

10.127.0.10 idrac-zygarde
10.127.0.11 idrac-umbreon
10.127.0.12 idrac-zubat
";
}
