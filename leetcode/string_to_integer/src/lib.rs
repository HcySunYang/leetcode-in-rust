struct Solution;
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut iter = s.chars().skip_while(|c| c.is_whitespace());
        let mut ans: i32 = 0;

        let [sign, should_consume] = match iter.clone().peekable().peek() {
            Some('-') => [-1, 1],
            Some('+') => [1, 1],
            _ => [1, 0],
        };

        if should_consume == 1 {
            iter.next();
        }

        let mut iter = iter.take_while(|c| c.is_digit(10));
        let overflow = if sign == 1 { i32::MAX } else { i32::MIN };
        while let Some(c) = iter.next() {
            let (finally_ans, o) = ans.overflowing_mul(10);
            if o {
                return overflow;
            }

            let (finally_ans, o) = finally_ans.overflowing_add(c.to_digit(10).unwrap() as i32);
            ans = finally_ans;

            if o {
                return overflow;
            }
        }

        sign * ans
    }

    // leetcode ans
    // pub fn my_atoi(string: String) -> i32 {
    //     let mut chrs = string.chars().skip_while(|c| c == &' ').peekable();

    //     let sign = if chrs.peek().map_or(false, |s| s == &'-') {
    //         chrs.next();
    //         -1i32
    //     } else {
    //         if chrs.peek().map_or(false, |s| s == &'+') {
    //             chrs.next();
    //         }

    //         1i32
    //     };

    //     chrs
    //         .into_iter()
    //         .take_while(|n| n.is_numeric())
    //         .try_fold(0i32, |acc, n| acc.checked_mul(10).and_then(|acc| acc.checked_add(n.to_digit(10).unwrap() as i32)))
    //         .map(|n| n * sign)
    //         .unwrap_or(if sign > 0 { std::i32::MAX } else { std::i32::MIN })
    // }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::my_atoi("-91283472332".to_owned()), -239);
    }
}
