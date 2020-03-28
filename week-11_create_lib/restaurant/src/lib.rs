// #[cfg(test)]
// pub mod front_of_house {
//     pub mod hosting{
//         pub fn add_to_waitlist(){

//         }
//         fn seat_at_table(){
            
//         }
//     }
//     mod serving{
//         fn take_order(){
            
//         }
//         fn serve_order(){
            
//         }
//         fn payment(){
            
//         }
//     }
//     // #[test]
//     // fn it_works() {
//     //     assert_eq!(2 + 2, 4);
//     // }
// }
// pub fn eat_at_restaurant(){
//     //absolute path
//     crate::front_of_house::hosting::add_to_waitlist();

//     //relative path
//     front_of_house::hosting::add_to_waitlist();
// }
// ============================
// // We can use direct functions here
// fn take_order(){
            
// }
// fn serve_order(){

// }
// fn payment(){

// }

// =====

// fn serve(){}
// mod hello{
    
    // mod back_of_house{
    //     fn fix_incorect_order(){
    //         cook();
    //         // serve();//errors
    //         super::serve();
    //         // super::super::serve();
    //     }
    //     fn cook(){}
    // }
// }

// ========

// mod back_of_house{
//     pub struct Breakfast{
//         pub toast : String,
//         fruit :String,
//     }

//     impl Breakfast {
//         pub fn summer(toast: &str) -> Breakfast{
//             Breakfast{
//                 toast:String::from(toast);
//                 fruit:String::from("peach");
//             }
//         }
//     }
    
//     let mut meal = back_of_house::Breakfast::summer("french");

//     meal.toast = String::from("Wheat");

//     println!("{}" , meal.toast);


// }

// // ==============use path =====

// mod front_of_house{
//     pub mod hosting {
//         pub fn wait(){}
//     }
// }

// //relative path
// use front_of_house::hosting;
// //absolute path
// // use crate::front_of_house::hosting;

// pub fn eat_at_restaurant(){
//     hosting::wait();
//     hosting::wait();
//     hosting::wait();
// }

 //// ============== disadvantage of direct using

// use std::fmt;
// use std::io;

// fn function() -> fmt::Result{

// }

// fn function() -> io::Result{

// }

// =========== solve error =========

// use std::fmt::Result as FmtResult;
// use std::io::Result as IoResult;

// fn function() -> fmt::Result as FmtResult{

// }

// fn function() -> io::IoResult{

// }

// ============ use public 


// pub use std::fmt;
// pub use std::io;

// fn function() -> fmt::Result{

// }

// fn function() -> io::Result{

// }
// =================


// mod front_of_house;

// pub use crate::front_of_house::hosting;

// pub fn eat_at_restaurant(){
//     hosting::wait();
//     hosting::wait();
//     hosting::wait();
// }

