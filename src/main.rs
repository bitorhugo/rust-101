
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
    println!(" Array element 0 -> {}", _a[0]);
}
