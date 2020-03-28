use std::io;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn sum(x: Rectangle) -> u32{
        x.width + x.height
    }
}
#[derive(Debug)]
struct Student{
    name: String,
    email: String,
    phone_no: u32,
    gender: String
}

#[derive(Debug)]
 struct Triangle {
    length1:u32,
    length2:u32,
    length3:u32,
}

impl Triangle{
    fn sum_tri(&self) -> u32{
        let sum:u32 = self.length1+self.length2+self.length3; 
        sum
    }

    fn checkSide(a : Triangle){
        if a.length1 > a.length2 && a.length1 > a.length3 {
            println!("\nSide A is greater which value is {}" , a.length1);
        } else if a.length2 > a.length1 && a.length2 > a.length3 {
            println!("\nSide B is greater which value is {}" , a.length2);
        } else if a.length3 > a.length1 && a.length3 > a.length2 {
            println!("\nSide C is greater which value is {}" , a.length3);
        } 
    }
}

#[derive(Debug)]
struct Person {
    name:String,
    Age:u32,
    country:String,
}

fn main() {
    // q1 in Answers.txt
    // q2 ->
    let mut s = String::from("Pakistan");
    println!("\n{}" , s);
    concatinate(&mut s);
    println!("\n{}" , s);
    // q3 -> imported io
    println!("\nenter First number : ");
    let mut a = String::new();
    io::stdin().read_line(&mut a);
    println!("\nenter Second number : ");
    let mut b = String::new();
    io::stdin().read_line(&mut b);
    println!("\nenter Third number : ");
    let mut c = String::new();
    io::stdin().read_line(&mut c);
    let a:u32 = a.trim().parse().unwrap();
    let b:u32 = b.trim().parse().unwrap();
    let c:u32 = c.trim().parse().unwrap();
    println!("\nAverage is : {}" , (a+b+c)/3 );
    // q4 -> 
    println!("\nenter String : ");
    let mut variable = String::new();
    io::stdin().read_line(&mut variable);
    // let i = ;
    println!("\nString Length is {:?}" , Input::checkLength(&mut variable) );
    
    // q5->
    println!("\nenter Number for Pyramid : ");
    let mut p_n = String::new();
    io::stdin().read_line(&mut p_n);
    let p_n:i32 = p_n.trim().parse().unwrap();
    for i in 1..p_n + 1 {
        for j in 1..i+1 {
            print!("*");    
        }
        println!();
    }

    // q6 -> 
    let st1 = Student{
        name: String::from("uzair"),
        email: String::from("uzair@dsa.com"),
        phone_no: 12345678,
        gender: String::from("male"),
    
    };
    let st2 = Student{
        name: String::from("saeed"),
        email: String::from("saeed@dsa.com"),
        phone_no: 87654321,
        gender: String::from("male"),
    };
    println!("{}", st1.email);
    println!("{:#?}", st2);

    // q7 ->
    let mut rect1 = Rectangle{
        width: 50,
        height: 100,
    };
    rect1.width = 150;
    println!("\n Width of rectangle changes from 50 to {}",rect1.width);

    // q8 ->
    // rec1 instance is already made in previous question
    println!("{:?}" , Rectangle::sum(rect1) );



    // q9 ->
    let tr1 = Triangle {
        length1 : 25 ,
        length2 : 60 ,
        length3 : 80 ,
    };
    println!("\nArea of Triangle is {}" , tr1.sum_tri());
    Triangle::checkSide(tr1);
    // q10 ->
    




}
fn concatinate(x: &mut String){
    x.push_str(" ZINDAbAD");
}
#[derive(Debug)]
struct Input{
}
impl Input{
    fn checkLength(x: &mut String) -> usize {
        println!("{}" , x.len()-1);
        let i;
        i = x.len();
        // x.len();

        i
    }   
}