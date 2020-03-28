//Struct
fn main() {
    // let mut a = String::from("hello");
    // my_fn(&mut a);
    // println!("{}" , a);

    let user1 = User{
        name: String::from("Uzair"),
        email: String::from("uzair@dsa.com"),
        phone_number: 03325689541,
        city: String::from("KHI"),
        online_status: true 
    }; // NOW user1 is Instance

    let user2 = User{
        name: String::from("Saeed"),
        email: String::from("saeed@dsa.com"),
        phone_number: 03325689541,
        city: String::from("KWT"),
        online_status: user1.online_status
    }; // NOW user2 is Instance

    let user3 = User{
        name: String::from("Saeed"),
        email: String::from("saeed@dsa.com"),
        phone_number: 03325689541,
        ..user2 
    }; // NOW user3 is Instance

    // println!("{:#?}" , user2);
    // println!("{}" , user2.name);

    //EMPLOYEE INSTANCES
    // let emp1 = Employee{
    //     name:String::from("Uzair"),
    //     hire_date:String::from("2019-08-01"),
    //     age:25,
    //     salary: 60000
    // };
    // let mut emp2 = Employee{
    //     name:String::from("DSA"),
    //     ..emp1
    // };
    // emp2.name = String::from("Saeed");
    // println!("{}" , emp2.name);

    //Function Make variable through struct
    // let emp3 = build_user(String::from("Asad") , 23 , String::from("2019-08-01"));
    // println!("{:#?}" , emp3);

    // //TuplE STRUCT (not used specifically)
    // let black = Color(0,5,0);
    // let origin = Point(0,0,0);
    // println!("{}" , black.1);
    // println!("{}" , origin.0);


}


//TUPLE STRUCT
struct Color(i32,i32,i32);
struct Point(i32,i32,i32);

fn build_user(a:String , b:i8 , c:String ) -> Employee{
    Employee {
        name:a,
        age:b,
        salary:3000, // Fix pre defined
        hire_date:c        
    }
}

fn my_fn(x: &mut String){
    x.push_str("world");
}

#[derive(Debug)]
struct Employee {
    name:String,
    hire_date:String,
    age:i8,
    salary:i32
}

#[derive(Debug)]
struct User{
    name:String,
    email:String,
    phone_number:u32,
    city:String,
    online_status:bool
}

// reference Struct
#[derive(Debug)]
struct Users{
    name:&str,
    email:&str,
} // errror will be life time prams which we'll study on next classes