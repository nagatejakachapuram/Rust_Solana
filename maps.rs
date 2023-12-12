// maps are objects used to store data in key value pairing
use std::collections::HashMap;
pub fn main() {
    let mut map = HashMap::new();
    // 3 operations on maps are
    // INSERT
    // GET
    // REOVE

    map.insert(0,"Hi_there");
    map.insert(1,"Bye");
    println!("{:?}",map);
    // pass only references since it's hash
    match map.get(&0){ // &0 is the reference
    Some(str) =>  println!("{}",str),
    _ => println!("None exists"),
    }
    match map.get(&1){
        Some(str) => println!("{}",str), // matching if there is string in 0 with &0 pointer as reference
        // if match found print the string.
        None => println!("None exixts"),

    }
    map.remove(&0);
    println!("{:?}",map);

}