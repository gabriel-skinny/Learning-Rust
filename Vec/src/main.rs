
#[derive(Debug)]
enum MultipleTypes {
    Int(i32),
    Text(String),
    Float(f64)
}

fn main() {
    let mut my_vec = vec![1,2, 3];    
    
    let multiple_types_vec = vec!(
        MultipleTypes::Int(40),
        MultipleTypes::Text("Teste alo".to_string()),
        MultipleTypes::Float(5.23213)
        );
    
    //let second_element = &mut my_vec[2];
    my_vec.push(10);
   
    for i in &mut my_vec {
        if i > &mut 8 {
            *i += 10;
        }
        println!("Element: {}", i);
    }

    println!("After loop {}", my_vec[2]);
    
    for i in &multiple_types_vec {
        match i {
            MultipleTypes::Int(val) => println!("Elements of my multiples type vec {}", val),
            MultipleTypes::Text(text) => println!("Elements of my multiples type vec {}", text),
            MultipleTypes::Float(val) => println!("Elements of my multiples type vec {}", val)
        }
    }

    //println!("Vec element in 2 \n is {}", second_element);
}
