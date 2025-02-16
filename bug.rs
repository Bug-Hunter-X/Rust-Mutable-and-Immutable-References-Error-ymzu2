fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &y;    // z is an immutable reference to y

    *y += 1;  // Modifying x through y is fine
    println!("x = {}", x); // Output: x = 6

    // The following line causes a compile-time error:
    //*z += 1; // Cannot modify x through z because z is immutable
}