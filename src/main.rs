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

   let s = String::from("Hello");
   let r1 = &s;
   println!("{}", r1);
   println!("{r1}");

}
