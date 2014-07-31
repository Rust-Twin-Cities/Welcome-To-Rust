fn calculate(expression: &str) -> int {
    ...
}



#[cfg(test)]
mod test {
    use super::calculate;

    fn test_simple() {
        assert_eq!(calculate("1 + 4 * 6"), 25i);
    }
    
    fn test_parens() {
        assert_eq!(calculate("(1 + 4) * 6"), 30i);
    }
}
