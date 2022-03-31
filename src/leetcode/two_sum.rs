impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut i: i32 = 0;
        while i < nums.len() {
            let mut j: i32 = i + 1;
            while j < nums.len() {
                if nums[i] + nums[j] == target {
                    let result: Vec<i32> = vec![i, j];

                    return result;
                }

                j = j + 1;
            }

            i = i + 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        let test_cases = vec![1, 2];
        let want = vec![1, 2];

        for (i, test_case) in test_cases {
            assert_eq!(want[i], test_case);
        }
    }
}
