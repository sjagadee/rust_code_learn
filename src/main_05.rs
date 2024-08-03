fn change_str(text: &mut String) {
    text.push('!');
}


fn main() {

    // Does work
    let mut a: String = String::from("Hello World");
    let b: &mut String = &mut a;
    // b.push('!');
    // println!("{}", a);
    // println!("{}", b);
    change_str(b);
    println!("{}", b);
    println!("{}", a);
}