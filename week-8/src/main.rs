fn main() {

    // let mut s10 = String::from("hi");
    // s10.push_str("!!");
    // println!("{}" ,  s10);
    
    // primitive = deep-copy
    // let s1 = 30;
    // take_owner(&s1);
    // println!("{}" , s1);

    // non-primitive = move-copy
    // let s1 = String::from("hello");
    
    // let len = calculate_length(s1);
    // println!("{}" , len );

    // Immutable Reference
    // let len = calculate_length(&s1);
    // println!("{} is the length of {}" , len , s1 );

    // // Mutable Referencing
    // let mut s = String::from("hello");
    // // println!("{}" , s);
    // change(&mut s);
    // println!("{}" , s);

    // //Restriction
    // let ref_to_nothing = dangle();
    // println!("{}" , ref_to_nothing);

    
}

// //Restriction
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

// // ==================Mutablecf==================
// fn change(some_variable: &mut String){
//     some_variable.push_str(" , world");
// }

//  ==============Immutale Reference=======================
// //fn calculate_length(s: String) -> usize{
// fn calculate_length(s: &String) -> usize{
//     // ';' must not be written because this is an expression 
//     // which saved/stored in a variable "len"
//     s.len()
//     // println!("{}" , s.len());
// }

// ======================OWNER-SHIp=========================
// fn take_owner(x:&String){
// // fn take_owner(x:i32){
//     println!("{}" , x);
// }
