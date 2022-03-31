pub struct Solution {}

impl Solution {
    // 暴力求解
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut i = 0;
        while i < nums.len() {
            let mut j = i + 1;
            while j < nums.len() {
                if nums[i] + nums[j] == target {
                    return vec![i as i32, j as i32];
                }

                j = j + 1;
            }

            i = i + 1;
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 测试用例数据结构
    #[derive(Debug)]
    struct TestCase {
        nums: Vec<i32>,
        target: i32,
        want: Vec<i32>,
    }

    #[test]
    fn test_two_sum() {
        // 测试用例
        let test_cases: Vec<TestCase> = vec![
            TestCase {
                nums: vec![2, 7, 11, 15],
                target: 9,
                want: vec![0, 1],
            },
            TestCase {
                nums: vec![3, 2, 4],
                target: 6,
                want: vec![1, 2],
            },
            TestCase {
                nums: vec![3, 3],
                target: 6,
                want: vec![0, 1],
            },
        ];

        for (case_index, test_case) in test_cases.iter().enumerate() {
            let got = Solution::two_sum(test_case.nums.clone(), test_case.target);

            assert_eq!(
                got, test_case.want,
                "\ncase_index: {}, test_case: {:?}\ngot: {:?}, want: {:?}\n",
                case_index, test_case, got, test_case.want
            );
        }
    }
}
