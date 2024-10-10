/// https://leetcode.com/problems/maximum-average-subarray-i/
///
/// PARAMS:
/// nums array of numbers
/// k length of subarray
/// 1 <= k <= n <= 105
///
///
/// RETURN:
/// float maxium average of subarray
///
///
/// SOLUTION:
/// Sliding window
/// Create window out of first elements
/// Iterate over starting from the end of the window
/// Save max average on each step
///
/// Example:
/// nums=[1,2,3], k=2
/// sum=3, w - prev_first_elem + next_elem
/// sum=5

fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
    let mut max_sum: i32 = nums.iter().take(k as usize).sum();
    let mut sum = max_sum;

    for i in k as usize..nums.len() {
        sum = sum - nums[i - k as usize] + nums[i];
        max_sum = max_sum.max(sum)
    }

    max_sum as f64 / k as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_max_average() {
        assert_eq!(find_max_average(vec![1, 2, 3], 2), 2.5); // max average of 2 elements is (2+3)/2
        assert_eq!(find_max_average(vec![1, 2, 3, 4, 5], 3), 4.0); // max average of 3 elements is (3+4+5)/3
        assert_eq!(find_max_average(vec![1, 1, 1, 1, 1], 2), 1.0); // max average of 2 elements is (1+1)/2
        assert_eq!(find_max_average(vec![], 2), 0.0); // empty vector returns 0.0
        assert_eq!(find_max_average(vec![1], 1), 1.0); // single element vector returns the element itself
        assert_eq!(find_max_average(vec![0, 4, 0, 3, 2], 1), 4.0); // single element vector returns the element itself
        assert_eq!(find_max_average(vec![4, 2, 1, 3, 3], 2), 3.0); // single element vector returns the element itself
    }
}
