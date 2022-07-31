pub fn max_profit(prices: &[i32]) -> i32 {
    let mut total = 0;
    let n = prices.len();

    for i in 0..n - 1 {
        if prices[i] < prices[i + 1] {
            total += prices[i + 1] - prices[i];
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_profit() {
        let prices = vec![7, 1, 5, 3, 6, 4];
        assert_eq!(7, max_profit(&prices));
    }

    #[test]
    fn test_max_profit_two() {
        let prices = vec![7, 6, 4, 3, 1];
        assert_eq!(0, max_profit(&prices));
    }

    #[test]
    fn test_max_profit_three() {
        let prices = vec![1, 2, 3, 4, 5];
        assert_eq!(4, max_profit(&prices));
    }
}
