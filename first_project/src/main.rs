// Rust is expression based
// fn main() {
//     println!("Hello, world!");
// }
// fn ssmain()
// {
//     let decimal_value =50:
//     let hex_value = 0x50;
//     let octal_value = 0o50;
//     let binary_value = 0b50;
//     let character_value =b'B'; //store in UTF8

//     println!(decimal_value);

// }

// //print emoji
// fn main(){
//     let emoji = 'u/{lf600}';
//     println!(emoji);
// }

//floating point
fn main(){
    //default datatype pi:f64 = float 64 bit , and for int i322 bydefault
    // let pi = 3.142;

    // binary 
    // let b = true;
    // println!("{}" ,b as i32);
    
    // overflow value error
    // let a:u8 = 10000;
    // println!("{}" , a);

    // print ascii code
    // let a = b'a';
    // println!("{}",a);

    // print octal
    // let a = b'a';
    // println!("{:o}",a);

    // print hex
    // let a = b'a';
    // println!("{:x}",a);

    // operations
    // datatype will be same for 2 or more variables
    // let x = 10;
    // let y = 5;

    // let add = x+y;
    // let sub = x-y;
    // let mul = x*y;
    // let div = x/y;
    // let reman = x%y;

    // println!("{}", add);
    // println!("{}", sub);
    // println!("{}", mul);
    // println!("{}", div);
    // println!("{}", reman);

    // trick
    // let x = 10;
    // let y = 4;
    // // let result:f64 = x/y;
    // let result = (x as f64) / (y as f64);

    // println!)("{}" , result);  

    // tuple cant be increase or decrese and 
    // value can be hetrogenuis means values datatype will be different
    // look like array
    // restruction and index access
    
    // let tuple = (1.5 , 1 , "hello");
    // // let tuple:(f64,f64,i8) = (1.5 , 1 , "hello");
    // let first_element = tuple.0;
    // let (x,y,z) = tuple;

    // println!("{:?}" , tuple);
    // println!("{}" , first_element);
    // println!("{}" , tuple.2);
    // println!("{}", y);

    // Arrays
    // array value datatype always same and 
    // values are homogenius
    // if array size is 5 then all 5 value will be given 
    // let products = ["item1" , "item2" , "item3" , "item4" , "item5"];
    // let a:[i64; 5] = [56,56,45,89,45];
    // let first_element = products[0];

    // // for empty values 
    // let b = [0; 5];
    // println!("{:?}" , b);
    // println!("{}" , products[4]);
    // println!("{}" , first_element);
    // println!("{:?}" , products);


    // FUNCTIONS
    // print_string(); //snake case naming technique
    // let x = 10;
    // let y = 20;
    // // let z = x+y;
    // add(x,y);

    // let return_value = added();
    // println!("{}", return_value);

    // let x = 5;
    // let y = {
    //             let x = 3;
    //             x+1 //samicolun will not use in last line 
    //         };

    // println!("{}" , y);

    // {
    //     let a = 3;
    //     let b = 2;
    //     println!("{}" , a);
    //     a + 25; //30 semicolon will be type in last line of block
    // }

    //IF ELSE EXPRESSION

    //t_temprature is threshold
    // let t_temprature = 30;
    // let input = 25;

    // if input < 20{
    //     println!("it is cold");
    // }
    // if input > 20 && input < 30 {
    //     println!("it is good weather");
    // }
    // if input > 30 {
    //     println!("it is hot");
    // }

    // ?????????????
    // let input = 25;

    // if input < 20{
    //     println!("it is cold");
    // }
    // else if input > 20 && input < 30 {
    //     println!("it is good weather");
    // }
    // else {
    //     println!("it is hot");
    // }

    // Array of tuple 
    let arr_tup = [(1,2,3),(4,5,6)];

    for element in arr_tup.iter(){
        println!("{}" , element.0);
        println!("{}" , element.1);
        println!("{}" , element.2); 
    }
    
}


fn print_string(){
    println!("hey\n");
    println!("hey1");
    println!("hey2");
    println!("hey3");
    
}

//prams datatype must be defined
fn add (x:i32 , y:i32){
    println!("{}" ,x+y);
}

//return u32
fn added()->u32{
    5+6
}
