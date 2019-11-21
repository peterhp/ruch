fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod unit_test {
    #[test]
    fn client_works() {
        assert_eq!(1 + 1, 2);
    }
}
