pub fn subtract_10(num: u32) -> u32 {
    return num - 10;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn minus_10_test() {
        let x = 100;
        let y = subtract_10(x);
        println!("x and y is from test: {} {}", x, y);
        assert_eq!(y, 90);
    }
}
