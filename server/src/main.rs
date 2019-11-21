fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod unit_test {
    #[test]
    fn server_works() {
        assert_eq!(2 + 2, 4);
    }
}
