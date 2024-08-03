// // will nor work
// fn make_string_dangle() -> &String {
//     let actual: String = String::from("dangle");
//     let dangle: &String = &actual;
//     return dangle;
// }

fn make_string_not_dangle() -> String {
    let actual: String = String::from("not dangle");
    return actual;
}

fn main() {

    // works
    let x: i32 = 55;
    let y: i32 = x;
    println!("{}", x);
    println!("{}", y);
    
    // // Does not work
    // let a: String = String::from("Hello World");
    // let b: String = a;
    // println!("{}", a);
    
    // clone - works
    let a: String = String::from("Hello World");
    let b: String = a.clone();
    println!("{}", a);
    
    // works
    let a: String = String::from("New Hello");
    let b: &String = &a;
    println!("{}", a);
    
    let dan: String = make_string_not_dangle();
    println!("{}", dan);
    
}