pub fn add_five(num: u32) -> u32 {
    return num + 5;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add_five_test() {
        let x = 100;
        let y = add_five(x);
        println!("x and y is from test: {} {}", x, y);
        assert_eq!(y, 105);
    }
}
