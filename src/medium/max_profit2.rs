/// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-ii/
///
/// PARAMS:
/// prices array of integers
///
/// each day buy and/or sell
/// can hold only one
/// can buy/sell on the same day
///
///
/// RETURN:
/// integer maximum profit
///
///
/// SOLUTION:
/// greedy algo
/// fold over window of 2 elements
/// if profit was made (curr_day - prev_day), add it acc profit

fn max_profit(prices: Vec<i32>) -> i32 {
    prices
        .windows(2)
        .fold(0, |acc, w| acc + i32::max(w[1] - w[0], 0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 7);
        assert_eq!(max_profit(vec![1, 2, 3, 4, 5]), 4);
        assert_eq!(max_profit(vec![7, 6, 4, 3, 1]), 0);
    }
}
