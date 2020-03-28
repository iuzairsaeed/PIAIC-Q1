fn main() {

    // let s = String::from("zairab");
    // let s = String::new();
    
    let s = "hello";
    let mut data = s.to_string();
    let data_2 = "uzair".to_string();
    
    data.push('s');
    println!("Stored in ; stack/literal = {:?}, heap = {:?},{:?}",s ,data,data_2 );

    let s1 = String::from("Piaic1");
    let s2 = String::from("Piaic2");
    println!("{}", s1+&s2 );
    println!("{}", s2);
    // println!("{}", &s1 );

    let a1 = String::from("Piaic1");
    let a2 = String::from("Piaic2");

    let x = format!("{}-{}-2019" ,a1 ,a2 );
    println!("{:?}", x);

    let len = String::from("Hola").len();
    println!("{}", len);

    let b = "pakistan";
    // let b = "chinese_language %#@";
    let d = &b[0..2];
    println!("{}", d);

    for c in "zairab".chars(){
        println!("{}", c);
    }

    for c in "zairab".bytes(){
        println!("{}", c);
    }
}