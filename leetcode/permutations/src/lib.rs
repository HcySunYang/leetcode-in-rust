#![allow(overflowing_literals)]
#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = vec![];
        let mut nums = nums;

        fn dfs(
            depth: u32,
            current: &mut Vec<i32>,
            ans: &mut Vec<Vec<i32>>,
            nums: &Vec<i32>,
            used: u32,
        ) {
            if depth as usize == nums.len() {
                ans.push(current.to_vec());
                return;
            }

            for idx in 0..nums.len() {
                if ((1 << idx) & used) != 0 {
                    continue;
                }

                current.push(nums[idx]);
                dfs(depth + 1, current, ans, nums, (1 << idx) | used);
                current.pop();
            }
        }

        dfs(0, &mut [].to_vec() as &mut Vec<i32>, &mut ans, &mut nums, 0);

        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        let ans = Solution::permute([1, 2, 3].to_vec());
        assert_eq!(
            ans,
            [
                [1, 2, 3],
                [1, 3, 2],
                [2, 1, 3],
                [2, 3, 1],
                [3, 1, 2],
                [3, 2, 1]
            ]
            .to_vec()
        );
    }
}
