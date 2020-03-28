
extern crate calculate;
use calculate::calculator_functions::basic_functions;
use calculate::calculator_functions::power_functions; 
pub fn main() {
    let mut a;
    a = basic_functions::add(5,2);
    println!("5 + 2 = {}", a );
    a = basic_functions::substract(100, 50);
    println!("100 - 50 = {}", a );
    a = basic_functions::divide(300, 100);
    println!("300 - 100 = {}", a );
    a = basic_functions::multiply(3, 5);
    println!("3 x 5 = {}", a );

    a = power_functions::square_function(8);
    println!("square of 8 is {}", a );
    a = power_functions::cube_function(2);
    println!("cube of 2 is {}", a );
    a = power_functions::power_function(2,4);
    println!("base is 2 and power is 4 result = {}", a );

}