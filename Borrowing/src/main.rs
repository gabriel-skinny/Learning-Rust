fn main() {
    let mut s = String::from("hello");
    
    let r1 = &s; // Borrowed for reading
    let r2 = &s; // Borrowed for reading  

    borrowed_string(r1);

    println!("String borrowed {} other string {}", r1, r2); // End of reference scope because is not used

    let r3mut = &mut s; // Can mutate S through reference
    
    mutate_string(r3mut);

    //let reference_to_nothing = dangle_pointer(); // Dangle goes out of scope because value is not moved only 
                                                 // only reference is retured

    let rigth_return = moved_value(); // Has access because rigth_return has ownership of moved_string

    println!("Moved value: {}", rigth_return);

    println!("S mutated {}", r3mut);
}

fn borrowed_string(borrow: &String) {
 
    println!("Borrowed string {}", borrow);

}

fn mutate_string(mutate: &mut String){
    mutate.push_str("Changed");
}

//fn dangle_pointer() -> &String {
//    let dangle = String::from("Dangled pointer");

//   &dangle
//}

fn moved_value() -> String {
    let moved_string = String::from("Moved");

    moved_string
}
