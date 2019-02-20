pub struct Manager<'a> {
    pub wbyl: std::vec::Vec<std::vec::Vec<&'a str>>
}

impl <'a> Manager<'a>{

    pub fn new (contents: &'a str) -> Manager {
        Manager{
            wbyl: words_by_line(&contents)
        }
    }

    pub fn list(&'a self) -> String{
        let mut out = String::new();
        for i in &self.wbyl {
            for j in i {
                out.push_str(&format!("{} ", j));
            }
            if out.ends_with(" "){
                out = out.trim().to_string();
            }
            out.push_str("\n");
        }
        out
    }


    pub fn add(&'a self, ipaddr:  &str, hostname:  &str){
        for i in &self.wbyl {
            if i[0] == ipaddr{
                println!("insert here");
                print!("{} ", ipaddr);
                println!("{} ", hostname);
            }else{
                for j in i{
                    print!("{} ", j);
                }
            }
            println!("");
        }
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
        let  contents = "";
        let manager = Manager::new(contents);
        let out = manager.list();
        assert_eq!(out,"");
    }
    #[test]
    fn test_sample_list() {
        let contents = SAMPLEDATA;
        let manager = Manager::new(contents);
        let out = manager.list();
        assert_eq!(out,SAMPLEDATA);
    }


    #[test]
    fn test_empty_add() {
        let contents = "";
        let manager = Manager::new(&contents);
        manager.add("127.0.0.0", "slashdot.org");
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
