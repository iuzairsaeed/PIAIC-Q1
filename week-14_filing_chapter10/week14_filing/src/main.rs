// use std::fs::File;
// use std::io::Read;
// use std::io::ErrorKind;
use std::io;
use std::collections::HashMap;

fn print_karo(name : String , times : i32){
    for i in 1..times+1{
        println!("{}",name );
    }
}
fn main() {
    // let mut name = String::new();
    // let mut times = String::new();
    // let string = io::stdin().read_line(&mut name);
    // io::stdin().read_line(&mut times);
    // let times:i32 = times.trim().parse().unwrap();
    // print_karo(name, times);
    // let strings = "uzairsaeed";
    // println!("{:?}", string );


    // // Hashing Revise
    // data will be tore with key and value
    // let mut points_table = HashMap::new();
    // points_table.insert(String::from("KK") , 10);
    // points_table.insert(String::from("LQ") , 9);
    // println!("{:#?}", points_table );

    // let key = String::from("LQ");
    // let result  = points_table.get(key);
    // // let result  = points_table.get("KK");
    // println!("{:#?}", result.unwrap() );



    //Task Activity 1
        let mut p_n = String::new();
        let mut v = vec![];

        // let mut v:Vec<String> = Vec::new();
        // let mut v = vec!["input"];
        // let p_n:i32 = p_n.trim().parse().unwrap();
       
        for i in 1..6 {
            // let input = io::stdin().read_line(&mut p_n);
            // io::stdin().read_line(&mut p_n);
            io::stdin().read_line(&mut p_n);
            let p_n:String = p_n.trim().parse().unwrap();
            v.push(p_n);
        }
        // for i in 0..5 {
        //     // let input = io::stdin().read_line(&mut p_n);
        //     // v.push(io::stdin().read_line(&mut p_n));
        //     println!("{:?}" , v.get(i));
        // }
        println!("{:?}" , v);



    // unwrap concept
    // let v = vec![];
    // let result = v.get(0);
    println!("{:?}", v );
    // println!("{:?}", result.unwrap() );


    // let a = [10,2,30,4];
    // // loop with the length of 'a' of element
    // for i in a.iter(){
    //     println!("{:?}", a);
    // }
    // // loop with the given range of number
    // for j in (1..2+1).rev(){
    //     println!("{:?}", j);
    // }    
    

    //Filing
    // let f = File::open("dsa.txt");
    // let f = File::open("dsa.txt").unwrap();
    // println!("{:?}", f );

    // let f1 = f.unwrap();

    // println!("{:?}", f1 );

    // match f {
    //     Ok(file) => file,
    //     // Err(error) => panic!("error is {:?}", error),
    //     Err(error) => match error.kind() {
    //         // else create new file if not found  (error kind  = not ofunf file)
    //     }
    // };
}
