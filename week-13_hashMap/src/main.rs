use std::collections::HashMap;
fn main() {
    // example 1
    // let team_name = vec![String::from("uzair"),String::from("Saeed")];
    // let team_point = vec!
    
    // example 2
    // let mut map = HashMap::new();
    // map.insert(10, String::from("a"));
    // map.insert(20, String::from("b"));
    // map.insert(30, String::from("c"));
    // map.insert(40, String::from("d"));
    // println!("{:?}", map );

    // for (key,value) in map {
    //     println!("key is {} and value is {}", key,value );
    // }

    // update map 
    // map.insert(30 , String::from("e"));
    // println!("{:?}", map );

    // check and update by key if value is not exist
    // map.entry(30).or_insert(String::from("if value is not save this statment"));
    // map.entry(50).or_insert(String::from("if value is not save this statment"));
    // println!("{:?}", map );

    //Split word with white space
    let text = "my name is uzair name is my";
    let mut map = HashMap::new();
    // for word in text.split_whitespace().rev(){
    for word in text.split_whitespace().rev(){
        // print!("{} ", word );
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map );

}
