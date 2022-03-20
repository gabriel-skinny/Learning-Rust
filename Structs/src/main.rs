
struct User {
    email: String,
    sing: u64,
    active: bool,
}

#[derive(Debug)]
struct Triangle {
    width: u32,
    heigth: u32,
}


impl Triangle {
    fn area(&self) -> u32 {
        self.width * self.heigth
    }

    fn can_hold(&self, compare_triangle: &Triangle) -> bool {
        self.heigth >= compare_triangle.heigth && self.width >= compare_triangle.width
    }

    fn make_square(size: u32) -> Triangle {
        Triangle {
            heigth: size,
            width: size,
        }
    }
}

fn main() {
    
    let first_user = User {
        email: String::from("Teste@gmail.com"),
        sing: 43,
        active: true,
    };

    println!("User {}", first_user.email);


    let second_user = User {
        active:false,
        ..first_user
    };

    println!("Second user email: {}", second_user.email);
   // println!("First user email: {}", first_user.email); Email moved to second_user

   let triangle = Triangle {
       heigth: 30,
       width: 50,
    };
    
    let second_triangle = Triangle {
        heigth: 20,
        width: 10,
    };
    
    let big_triangle = Triangle {
        heigth: 100,
        width: 200,
    };

   let response = triangle.area();
   println!("Calulate area {}", response);

   println!("Triangle heigth {:?}", triangle); 
   println!("Can holde {}", triangle.can_hold(&second_triangle));
   println!("Can hold {}", triangle.can_hold(&big_triangle));
   println!("My square {:?}", Triangle::make_square(35));
}

