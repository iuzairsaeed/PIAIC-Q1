fn main() {
    let mut v1:Vec<String> = Vec::new();
    println!("V1 is {:?}", v1);
    
    let v2 = vec![1,2,3];
    println!("{:?}", v2);
    
    let mut shopping_list = vec!["vegetables","snacks","meat"];
    shopping_list.push("chicken");
    println!("{:?}", shopping_list);
    shopping_list.pop();
    println!("{:?}", shopping_list);

    let second = &shopping_list[1];
    println!("second index of immutable shopping list {:?}" , second);

    let second = & mut shopping_list[1];
    println!("second index of mutable shopping list {:?}" , second);

    let second_shopping = shopping_list.get(1);
    println!("{:?}", second_shopping);

    match shopping_list.get(2){
        Some(a) => println!(" here is {}", a ),
        None => println!("No value")
    }

    #[derive(Debug)]
    enum Human{
        Height(i32),
        Name(String),
        Age(i32),
    };

    let mut human_vec : Vec<Human> =  Vec::new();
    human_vec.push(Human::Height(58));
    human_vec.push(Human::Name(String::from("Uzair")));
    human_vec.push(Human::Age(40));

    // println!("Human data = {:?}", human_vec )

    let mut vector_in_vector : Vec<Vec<i32>> = Vec::new();
    let inner_vector = vec![1,2,3];
    let inner_vector_2 = vec![4,5,6];
    vector_in_vector.push(inner_vector);
    vector_in_vector.push(inner_vector_2);
    println!("{:?}", vector_in_vector );

    let mut tuple_vector : Vec<(i32,String)> = Vec::new();
    // tuple_vector.push(1,"Uzair");
    // tuple_vector.push(2,"Saeed");

    let mut v = vec![10,20,30,40];

    for i in &v {
        println!("{}",i );
    }

    for i in &mut v {
        *i += 100;
    }
    // i use '&' beacause i want to print after without this ->'&' , 'v' is move to 'i'
    println!("{:?}", v)

    

}

