fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x

    *y += 1;  // Modifying x through y is fine
    println!("x = {}", x); // Output: x = 6
}

// Alternative solution (using a mutable reference directly):
fn main(){
    let mut x = 5;
    x += 1;
    println!("x = {}", x); // Output: x = 6
}