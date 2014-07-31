fn phonetics(message: &str) -> String {
    ...   
}

#[cfg(test)]
mod test {
    fn test_lower() {
        assert_eq!(phonetics("abc"), String::from_str("Alpha Bravo Charlie"))
    }
    
    fn test_upper() {
        assert_eq!(phonetics("ABC"), String::from_str("Alpha Bravo Charlie"))
    }
}
