pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::add;
    #[test]
    fn it_works() {
        assert_eq!(add(56, 64), 120);
    }
}
