#[derive(Debug)]
struct User {
    active: bool,
    email: String,
    username: String,
    sign_in_count: usize,
}

fn build_user (username: String, email: String) -> User {
    return User {
        email, 
        username,
        active : true,
        sign_in_count : 1,
    };
}
fn take_ownership (some_string : String) {
    println!("Current owner of {some_string}");
}

fn take_scalar (_num : isize) {
}

fn main () {
   let s = String::from("Hello World"); 
   // move s into take_ownership func
   take_ownership (s); // main loses ownership of s

   // not able to use s
   //let d = s; 
   let num = 5;
    //Since num implements the Copy trait nothing happens
   take_scalar (num);
   println!("{num}");

   let mut user = build_user(String::from("hugo"), String::from("email"));
   println!("{}", user.email);
   user.email = String::from("new email");
   println!("{}", user.email);
   println!("{:?}", user);
}

