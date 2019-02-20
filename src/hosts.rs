


pub struct Hosts {
    contents: String        
}

impl Hosts{

    pub fn new (contents:  String) -> Hosts {
        let s = contents.clone();
        let l = words_by_line(&contents);
        Hosts{
            contents: s,
                
        }
    }

    pub fn list(& self) -> String{
        self.contents.clone()        
    }


    pub fn add(& mut self, ipaddr:  &str, hostname:  &str){

        let mut wbyl =  words_by_line(&self.contents);

        for i in &wbyl {
            if i[0] == ipaddr{
                for j in i{
                    if j == &hostname{
                        //Found it. 
                        return
                    }
                }
            }
        }
        wbyl.push(vec!("ipaddr", "hostname"));

        
        let mut out: String = String::new();
        
        for i in &wbyl {
            for j in i {
                out.push_str(&format!("{} ", j));
            }
            if out.ends_with(" "){
                out = out.trim().to_string();
            }
            out.push_str("\n");
        }
        self.contents = out.clone();

        
    }
}

pub fn words_by_line<'a>(s: &'a str) -> Vec<Vec<&'a str>> {
    s.lines().map(|line| {
        line.split_whitespace().collect()
    }).collect()
}


#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_empty_add() {
        let contents = "".to_string();
        let mut manager = Hosts::new(contents);
        manager.add("127.0.0.0", "slashdot.org");
        manager.add("10.10.2.1", "tower");
        let out = manager.list();
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
