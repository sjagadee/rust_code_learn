
// Sotred in static memory
const MY_STR: &str = "Hello from CONSTANT";

fn main() {

    // heap
    let s: String = String::from("Hello World");
    println!("s is {}", s);
    
    // s_1 is on Stack
    // "Hello2" stored in string literal
    let s_1: &str = "Hello2";
    println!("s_1 is {}", s_1);
    
    // msg here is sotred in heap
    let msg: String = "New Hello".to_string();
    println!("msg is {}", msg);
    
    println!("constant in static memory {}", MY_STR);
    
}