fn main() {
    let mut a = 3;
    let mut b = 4;

    println!("a = {}, b = {}", a, b);

    a = a + b; // a = 3 + 4 = 7
    b = a - b; // b = 7 - 4 = 3
    a = a - b; // a = 7 - 3 = 4

    println!("a = {}, b = {}", a, b);
}
