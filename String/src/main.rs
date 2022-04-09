fn main() {
   let s1 = String::from("Gabriel");
   let s2 = String::from("do");
   let s3 = String::from("Barrabacoa");

   let result = format!("{}-{}-{}", s1, s2 ,s3);

   println!("Result: {}", result);

   println!("First string {}", s1);
    

   let result_add = s1 + "-" + &s2 + "-" + &s3;

   println!("Result: {}", result_add);
        
   //println!("First string {}", s1);


   let non_utf_8_string = String::from("Hola Que Tal");
    
   println!("\n");
   print!("NON UFT-8: ");
   for a in non_utf_8_string.chars() {
     print!("{}", a);
   }

    println!("\n");

    let utf_8_string = String::from("नमस्ते");
    
    let length = utf_8_string.len();
    
    print!("UFT-8: ");
    for b in utf_8_string.chars() {
        print!("{}", b);
    }

    println!("\n");
    
    println!("UTF-8 length: {}, NON UTF-8 length: {}", length, non_utf_8_string.len());
}
