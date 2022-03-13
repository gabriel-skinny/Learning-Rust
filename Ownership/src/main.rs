fn main() {
    let mut dest = String::new();

    teste(&mut dest);
    
    println!("test {}", dest);
    
    let moved_string = String::from("Moved");

    let back = teste2(moved_string);

    println!("Back here {}", back);
    // println!("Try print moved {}", movedString); Borrowed value
}

fn teste(dest: &mut String) {
    let firststring = String::from("Borrowedeste");
    
    *dest = firststring;
}

fn teste2(dest: String) -> String {
    println!("Moved to here {}", dest);
    
    return dest;
}

