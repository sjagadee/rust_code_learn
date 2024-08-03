fn main() {

    // Dereferencing coercion
    let mut a: String = String::from("Hello World");
    let b: &mut String = &mut a;
    
    *b = String::from("New World");
    println!("{}", a);
    
    let mut x: i32 = 55;
    dbg!(x);
    x = 500;
    dbg!(x);
    
    let y: &mut i32 = &mut x;
    // defrerncing coercion
    *y += 5;
    dbg!(y);
}