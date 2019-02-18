pub struct Manager<'a> {
    pub wbyl: std::vec::Vec<std::vec::Vec<&'a str>>
}


pub fn words_by_line<'a>(s: &'a str) -> Vec<Vec<&'a str>> {
    s.lines().map(|line| {
        line.split_whitespace().collect()
    }).collect()
}

//pub fn list(wbyl: std::vec::Vec<std::vec::Vec<&str>>){
pub fn list(manager: &Manager){
    for i in &manager.wbyl {
        for j in i{
            print!("{} ", j);
        }
        println!("");
    }
}

pub fn set(manager: &Manager, ipaddr:  &str, hostname:  &str){
    for i in &manager.wbyl {
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

#[cfg(test)]
mod tests {
     use super::*;
    #[test]
    fn test_empty_list() {
        let  contents = "";
        let manager = Manager{
            wbyl: words_by_line(&contents)
        };
        list(&manager);
    }

    #[test]
    fn test_empty_set() {
        let contents = "";
        let manager = Manager{
            wbyl: words_by_line(&contents)
        };
        set(&manager, "127.0.0.0", "slashdot.org");
    }
}
