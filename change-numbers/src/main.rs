fn main() {
    let mut a = 3;
    let mut b = 4;

    println!("a = {}, b = {}", a, b);

    a = a + b; // a = 30
    b = a - b; // b = 30 - 20 = 10
    a = a - b; // a = 30 - 10 = 20

    println!("a = {}, b = {}", a, b);
}
