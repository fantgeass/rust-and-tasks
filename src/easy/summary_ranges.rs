fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
    if nums.is_empty() {
        return vec![];
    }

    let mut result = vec![];
    let mut from = nums[0];
    let mut to = nums[0];

    for num in &nums[1..] {
        if to + 1 == *num {
            to = *num;
        } else {
            push(from, to, &mut result);
            from = *num;
            to = *num;
        }
    }
    push(from, to, &mut result);

    result
}

fn push(from: i32, to: i32, result: &mut Vec<String>) {
    let msg = if from == to {
        format!("{}", from)
    } else {
        format!("{}->{}", from, to)
    };
    result.push(msg);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_summary_ranges() {
        assert_eq!(
            summary_ranges(vec![0, 1, 2, 4, 5, 7]),
            vec!["0->2", "4->5", "7"]
        );

        assert_eq!(
            summary_ranges(vec![0, 2, 3, 4, 6, 8, 9]),
            vec!["0", "2->4", "6", "8->9"]
        );

        assert_eq!(summary_ranges(vec![]), Vec::<String>::new());
    }
}
