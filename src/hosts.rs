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
        let out = manager.list();
        assert_eq!(out,"127.0.0.0 slashdot.org\n");
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
