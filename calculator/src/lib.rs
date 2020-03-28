pub fn test() {
    println!("Test");
}
pub mod calculator_functions{
    pub mod basic_functions{
        pub fn add(x:i32 , y:i32)  -> i32 {
            let z:i32 = x+y;
            z
        }
        pub fn substract(x:i32 , y:i32) -> i32{
            let z:i32 = x-y;
            z
        }
        pub fn divide(x:i32 , y:i32) -> i32{
            let z:i32 = x/y;
            z
        }        
        pub fn multiply(x:i32 , y:i32) -> i32{
            let z:i32 = x*y;
            z
        }
    }
    pub mod power_functions{
        pub fn square_function(x:i32) -> i32{
            let z:i32 = x*x;
            z
        }
        pub fn cube_function(x:i32) -> i32{
            let z:i32 = x*x*x;
            z
        }
        
        pub fn power_function(base:i32 , power:i32) -> i32{
            let mut z:i32 = 1;
            for _i in 1..power+1{
                z = z*base;
            }
            z
        }
    }
}

