#![allow(overflowing_literals)]
#![allow(dead_code)]

struct Solution;
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut x = x;
        let mut ans: i32 = 0;
        loop {
            if x == 0 {
                break;
            }

            let digit = x % 10;
            x = x / 10;

            if ans.checked_mul(10).is_some() {
                ans = unsafe { ans.checked_mul(10).unwrap_unchecked() }
                    .checked_add(digit)
                    .unwrap_or(0)
            } else {
                return 0;
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::reverse(1534236469), 9646324351);
    }
}
