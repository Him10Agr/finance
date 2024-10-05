#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_present_value() {
        let pv = present_value(1000.0, 0.05, 10);
        assert_eq!(pv, 613.91);
    }
}