extern crate rust_boilerplate;

#[cfg(test)]
mod tests {
    use rust_boilerplate::example_42;

    #[test]
    fn example_42_test() {
        let result = example_42();
        assert_eq!(result, 42);
    }
}
