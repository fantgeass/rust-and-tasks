fn max_profit(prices: Vec<i32>) -> i32 {
    let mut buy = prices[0];
    let mut profit = 0;
    for i in 1..prices.len() {
        if prices[i] < buy {
            buy = prices[i];
        } else if prices[i] - buy > profit {
            profit = prices[i] - buy;
        }
    }
    profit
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_profit() {
        assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
        assert_eq!(max_profit(vec![7, 6, 4, 3, 1]), 0);
        assert_eq!(max_profit(vec![1, 6, 4, 6, 8]), 7);
        assert_eq!(max_profit(vec![1, 3, 5, 1, 3, 7]), 6);
    }
}
