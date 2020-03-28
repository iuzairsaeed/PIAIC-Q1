fn main() {
    // Stack (by default)
    // let x = 5;
    // // let x = String::from("PIAC");
    // // let y = "hello";

    // // another_function(6);

    // // let  z = 2.3;

    // dummy(x);
    // println!("{}" , x);

    // ---------

    // let s = "hello";
    // s1 is mutable & s isn't
    // let s1 = String::from("hello");

    // let s2 = s1;

    // println!("{}" , s2);    
    // println!("{}" , s1);


    // ----OWNERSHIP E.G--------
    
    // let x = String::from("hello");
    // {
    //     let y = String::from("insdie the scope");
    //     println!("{}" , y);
    // }
    // println!("{}" , x);
    
    let x = String::from("PIAIC");
    // let y =x;
    // y = PIAic
    // x = none / nothing / will show error / beacus x is no more owner

    // println!("{}" , y);
    another_func(x);
    // println!("{}" , x);
}

fn another_func(a:String){
    println!("{}" , a);
}
// fn dummy(a: i32){
    // let x = 10;
    // let y = 15;
// 
    // println!("{}" ,x);
    // println!("{}" ,y);
    // 
    // free(x);
    // free(y);
    // free(a);
// }

// fn another_function (x:i32){
//     let number: i8 =10;
//     let condition = true;
// }


//how float value is stored inmemory Assignment