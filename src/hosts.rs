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
