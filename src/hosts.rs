pub fn words_by_line<'a>(s: &'a str) -> Vec<Vec<&'a str>> {
    s.lines().map(|line| {
        line.split_whitespace().collect()
    }).collect()
}

pub fn list(wbyl: std::vec::Vec<std::vec::Vec<&str>>){
    for i in &wbyl {
        for j in i{
            print!("{} ", j);
        }
        println!("");
    }
}

pub fn set(wbyl: std::vec::Vec<std::vec::Vec<&str>>,
       ipaddr:  &str,
       hostname:  &str){
    for i in &wbyl {
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
        let  a = "";
        let wbyl: std::vec::Vec<std::vec::Vec<&str>> = words_by_line(a);
        list(wbyl);
    }

    #[test]
    fn test_empty_set() {
        let  a = "";
        let wbyl: std::vec::Vec<std::vec::Vec<&str>> = words_by_line(a);
        set(wbyl,"127.0.0.0", "slashdot.org");
    }
}
