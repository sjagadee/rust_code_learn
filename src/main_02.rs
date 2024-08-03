const MY_INTEGER: u8 = 10;

fn main() {
    
    // stack
    let x: u8 = 23;
    println!("x is {}",x);
    
    // heap
    let mut arr: Vec<u8> = vec![1,2,3,4,5];
    arr.push(10);
    println!("vector is {:?}", arr);
    
    // a reference on the stack pointing to a value on the heap
    let arr_2 = &arr[0..3];
    println!("arr2 is {:?}", arr_2);
    
    // heap
    let mut s: String = String::from("Srini");
    s.push(' ');
    s.push('!');
    println!("String is {}", s);
    
    // a reference on the stack pointing to a vlaue on the heap
    let s_2 = &s[0..2];
    println!("string 2 is {}", s_2);
    
    println!("MY_INT is {}", MY_INTEGER);
}