fn main() {
    let mut dest = String::new();

    teste(&mut dest);
    
    println!("test {}", dest);
    
    let moved_string = String::from("Moved");

    let back = teste2(moved_string);

    println!("Back here {}", back);
    // println!("Try print moved {}", movedString); Borrowed value

    let number = 5;
    teste_stack_value(number);
    println!("Number not moved because has copy type, number: {}", number);
    
    let literal_string = "string";
    teste_string_stack(&literal_string);
    println!("Also access here {}", literal_string);
}

fn teste_string_stack(string: &str) {
    println!("Copied here {}", string);

}

fn teste(dest: &mut String) {
    let firststring = String::from("Borrowedeste");
    
    *dest = firststring;
}

fn teste2(dest: String) -> String {
    println!("Moved to here {}", dest);
    
    return dest;
}

fn teste_stack_value(number: u32) {
    println!("Here the value too {}", number);

}
