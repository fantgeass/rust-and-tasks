/// https://leetcode.com/problems/find-closest-number-to-zero/

/// PARAMS:
/// nums: vector of integers
/// 1 <= length <= 1000
/// -10^5 <= nums[i] <= 10^5
///
///
/// RETURN:
/// integer
/// closest number to 0
/// if multiple answers, return the largest
///
///
/// SOLUTION:
/// - Reduce over nums
/// - Compare acc_num to current num and return closest to 0 by using comparison logic
///
/// Comparison logic:
/// Two RETURN conditions should be resolved.
/// We can do it if we use tuples with two elements: (num.abs(), -num)
/// First element is for "closest number to 0" condition
/// Second element is for "if multiple answers, return the largest" condition
///
/// Example:
/// acc = -1, num = 1
/// (1, 1) cmp (1, -1) = a > b
/// means we need to save b because -1 and 1 have same mangitude to 0, but 1 is larger than -1

fn find_closest_number(nums: Vec<i32>) -> i32 {
    nums.into_iter()
        .min_by_key(|num| (num.abs(), -num))
        .unwrap()
}

mod tests {
    use super::*;

    #[test]
    fn test_find_closest_number() {
        assert_eq!(find_closest_number(vec![-4, -2, 4, 8]), -2);
        assert_eq!(find_closest_number(vec![-1, 1]), 1);
        assert_eq!(find_closest_number(vec![2, -1, 1]), 1);
        assert_eq!(find_closest_number(vec![2, 1, 1, -1, 100000]), 1);
        assert_eq!(find_closest_number(vec![2, 1, 0, -1, 2]), 0);
    }
}
