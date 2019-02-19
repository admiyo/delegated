pub struct Manager<'a> {
    pub wbyl: std::vec::Vec<std::vec::Vec<&'a str>>
}

impl <'a> Manager<'a>{

    pub fn new (contents: &'a str) -> Manager {
        Manager{
            wbyl: words_by_line(&contents)
        } 
    }

    pub fn list(&'a self){
        for i in &self.wbyl {
            for j in i{
                print!("{} ", j);
            }
            println!("");
        }
    }


    pub fn set(&'a self, ipaddr:  &str, hostname:  &str){
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
        let manager = Manager{
            wbyl: words_by_line(&contents)
        };
        manager.list();
    }

    #[test]
    fn test_empty_set() {
        let contents = "";
        let manager = Manager::new(&contents);
        manager.set("127.0.0.0", "slashdot.org");
    }
}
