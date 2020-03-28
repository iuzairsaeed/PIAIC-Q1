// use std::io;

// #[derive(Debug)]
// struct Rectangle{
//     width:u32,
//     height:u32
// }

// #[derive(Debug)]
// struct Student{
//     name:String,
//     marks:u32,
//     suspended: bool,
// }

// struct Calculator{
//     // result:u32
// }

// impl Calculator{
//     fn add(a:u32,b:u32){
//         println!("Result is : {}" , a+b);
//     }
//     fn sub(a:u32,b:u32){
//         println!("Result is : {}" , a+b);
//     }
//     fn multiply(a:u32,b:u32){
//         println!("Result is : {}" , a+b);
//     }
//     fn divide(a:u32,b:u32){
//         println!("Result is : {}" , a+b);
//     }
// }
// impl Student{
//     fn print_marks(&self) -> u32{
//         // println!("{}" , self.marks);
//         self.marks
//     }
//     fn do_suspended(&mut self){
//         self.suspended = true;
//     }
//     fn change_marks(&mut self , new_marks : u32){
//         self.marks = new_marks;
//     }
//     fn calculate_average(a:u32 , b:u32 , c:u32) -> u32{
//         let sum = a+b+c;
//         let per = sum * 100 / 300;

//         per
//     }
// }


// impl Rectangle{
//     /// Where Area is a method of Rectangle
//     /// this is the difference from function
//     /// &self = rect1 , rect1.width or self.width is same
//     fn area(&self) ->u32{
//         self.width * self.height
//     }
//     // fn square(size:u32) -> Rectangle{
//     //     Rectangle {width:size , height:size};
//     // }
// }

// fn main() {
//     // let rect1 = Rectangle{width:30 , height:50};
//     // let rect2 = Rectangle{width:5 , height:5};
//     // println!("{} {}" , rect1.area() , rect2.area());

//     // let mut st1 = Student{
//     //     name : String::from("uzair"),
//     //     marks : 5,
//     //     suspended : false
//     // }; // Where st1 is a instance 


//     // println!("{}" , st1.print_marks());
//     // println!("{}" , st1.suspended);
//     // st1.do_suspended();
//     // st1.change_marks(12);
//     // println!("{}" , st1.print_marks());
//     // println!("{}" , st1.suspended);

//     ////Calling Associated Function 
//     ////struct::associatedFunction()
//     // println!("{}" , Student::calculate_average(80,95,73) );
//     println!("enter first number : ");
//     let mut a = String::new();
//     io::stdin().read_line(&mut a);
//     println!("enter second number : ");
//     let mut b = String::new();
//     io::stdin().read_line(&mut b);
//     let a:u32 = a.trim().parse().unwrap();
//     let b:u32 = b.trim().parse().unwrap();
//     // println!("first number is {}" , a+1);

//     Calculator::add(a,b);


// }
fn main() {
let x: i32 = 2; // or any other integer type

println!("{:?}" , assert_eq!(x.pow(5), 32))
}