use std::io;

struct User {
    active: bool,
    email: String,
    username: String,
    sign_in_count: usize,
}

// named tuple
struct Color (i16, i16, i16);

fn build_user (username: String, email: String) -> User {
    return User {
        email, 
        username,
        active = true,
        sign_in_count = 1,
    };
}

fn main() {
    // tuples
    let _a = (1, 2, 3);
    // tuple with type annotation
    let _b : (u8, char, bool) = (1, 'T', false);
    // pattern match with let
    let (_x, _y, z) = _b; // x, y and z are single values
    println!("Value of z : {z}");
    // we can also access tuple element via '.'
    println!("Value of second element of b : {}", _b.1);

    // arrays 
    let _a = [1, 2, 4];
    let _a = [1; 5]; // array with the element 1 repeated 5x
    let a : [usize; 5] = [0, 1, 2, 3, 4]; // array with size 5 of i32
    println!(" Array element 0 -> {}", a[0]);

    let mut index : usize;

    let _l = loop {
        println!("Enter a number");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input) // explicity say &mut otherwise we won't be able to modify the value
            .expect("Failed to read line"); 

        index = input.trim().parse().expect("number");

        if index < a.len() {
            break
        }
        println!("Out of bounds value");
    };

   println!("Element with index {} -> {}", index, a[index]);

    // returning loop values using break
   let a = 5; 
   let lop = loop {
       break a;
   };
   println!("{lop}");

   let mut s = "hello";

   let mut user1 = build_user("hugo", "email");
   println("{}", user1.email);
   let user2 = User {
       email: String::from("bruno"),
       ..user1
   };
   println!("{}", user2.email);
}
