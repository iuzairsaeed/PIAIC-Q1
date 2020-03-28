// =================q1==========
#[derive(Debug)]
enum Student {
    Onsite{Name:String , Roll_num : String},
    Online{Name:String , Roll_num : String},
}

fn main() {
    let st1 = Student::Onsite{ Name:String::from("Uzair") , Roll_num:String::from("R1")};
    let st2 = Student::Online{ Name:String::from("Saeed") , Roll_num:String::from("H1")};    
}
// ==============q2====================
#[derive(Debug)]
enum Vehicle{
    Cars,
    Truck,
    Bikes,
}
#[derive(Debug)]
struct License {
    name:String,
    vehicle:Vehicle,
}
fn main(){
    let cars = Vehicle::Cars;
    let bikes = Vehicle::Bikes;
    let Truck = Vehicle::Truck;

    let data_1 = License {
        name : String::from("Honda"),
        vehicle: Vehicle::Cars, 
    };
    println!("{:?}" , data_1);

}
// ============================q3========
// #[derive(Debug)]
// enum Vehicle{
//     Cars(String),
//     Truck(String),
//     Bikes(String),
// }
// fn main(){
//     let cars = Vehicle::Cars(String::from("Honda"));
//     let bikes = Vehicle::Bikes(String::from("Yamaha"));
//     let Truck = Vehicle::Truck(String::from("CAT"));

//     println!("{:?}" , cars);
//     println!("{:?}" , bikes);
//     println!("{:?}" , Truck);

// }
// ===========+Q4============
// #[derive(Debug)]
// enum Shape{
//     Circle(u32),
//     Triangle{a:u32,b:u32,c:u32},
//     Rectengle{a:u32,b:u32,c:u32,d:u32},
//     Square{a:u32,b:u32,c:u32,d:u32},
// }
// fn main(){
//     let shape1 = Shape::Circle(8);
//     let shape2 = Shape::Triangle{a:10 , b:20 , c: 30};
//     let shape3 = Shape::Rectengle{a:10 , b:20 , c: 30 , d:40 };
//     let shape4 = Shape::Square{a:10 , b:20 , c: 30 , d:40 };
//     shape1.call();
//     shape2.call();
//     shape3.call();
//     shape4.call();
// }

// impl Shape {
//     fn call(&self){
//         // match self{
//         //     Shape::Circle(data) => {
//         //         println!(" Shape is {:?} and its radius is {}" , self , data )
//         //     },
//         //     Shape::Rectengle{a:u32,b:u32,c:u32,d:u32} => {
//         //         println!(" Shape is {:?} and its sides are {:?} {:?} {:?} {:?}" , self ,a,b,c,d  )
//         //     },
//         //     _ => println!("Invalid"),
//         // }
//         println!("Shape is {:?}", self);
//     }
// }
// // ===================Q5=============
// 

// fn main(){
//     let some_num1= Some(150);
//     let some_num2 = Some(614.98);
//     let some_string1 = Some(String::from("How are you?"));
//     let null_value:Option<f64> = None;
//     println!("{:?}" ,some_num1);
//     println!("{:?}" ,some_num2);
//     println!("{:?}" ,some_string1);
//     println!("{:?}" , null_value);
// }

// ==========Q6================
// #[derive(Debug)]
// enum Laptop {
//     HP,
//     Dell(Series),
//     Asus,
//     Lenovo,
// }
// #[derive(Debug)]
// enum Series{
//     S_1000,
//     S_2000,
//     S_3000,
//     S_4000,
//     S_5000,
//     S_6000,
// }

// fn main(){
//     let lap1 = Laptop::Dell(Series::S_1000);
//     let lap2 = Laptop::Dell(Series::S_6000);
//     println!("{:#?}\n{:#?}" , lap1 , lap2);
// }

