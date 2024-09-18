/// https://leetcode.com/problems/find-closest-number-to-zero/

fn find_closest_number(nums: Vec<i32>) -> i32 {
    *nums
        .iter()
        .reduce(|acc, num| {
            if num.abs() < acc.abs() {
                num
            } else if num.abs() == acc.abs() {
                std::cmp::max(num, acc)
            } else {
                acc
            }
        })
        .unwrap()
}

mod tests {
    use super::*;

    #[test]
    fn test_find_closest_number() {
        assert_eq!(find_closest_number(vec![-4, -2, 4, 8]), -2);
        assert_eq!(find_closest_number(vec![-1, 1]), 1);
    }
}
