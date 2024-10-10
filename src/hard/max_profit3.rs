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

fn max_profit(prices: Vec<i32>) -> i32 {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    prices
        .windows(2)
        .fold(BinaryHeap::from([Reverse(0); 2]), |mut heap, w| {
            let profit = Reverse(i32::max(w[1] - w[0], 0));
            if profit < *heap.peek().unwrap() {
                heap.pop();
                heap.push(profit);
            }
            heap
        })
        .iter()
        .fold(0, |acc, rev_i| acc + rev_i.0)
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test() {
//         assert_eq!(max_profit(vec![3, 3, 5, 0, 0, 3, 1, 4]), 6);
//         assert_eq!(max_profit(vec![1, 2, 3, 4, 5]), 4);
//     }
// }
