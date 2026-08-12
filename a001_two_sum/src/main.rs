use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
       let mut s = HashSet::new();

       for n in &nums {
        if s.contains(&(target - n)) {
            let a = nums.iter().position(|&x| x == *n).unwrap();
            let b = nums.iter().rposition(|&x| x == target - *n).unwrap();
            return vec![a as i32,b as i32]
            //return vec![*n, target - *n]
        } else {
            s.insert(n);
        }
       }
       vec![0,0] 
    }
}

fn main() {

}


#[cfg(test)]
mod tests {
    use super::Solution;

    fn assert_two_sum_any_order(nums: Vec<i32>, target: i32, expected: [i32; 2]) {
        let mut actual = Solution::two_sum(nums, target);
        let mut expected = expected.to_vec();

        actual.sort_unstable();
        expected.sort_unstable();

        assert_eq!(actual, expected);
    }

    #[test]
    fn returns_indices_for_standard_case() {
        assert_two_sum_any_order(vec![2, 7, 11, 15], 9, [0, 1]);
    }

    #[test]
    fn returns_indices_for_duplicate_values() {
        assert_two_sum_any_order(vec![3, 3], 6, [0, 1]);
    }

    #[test]
    fn returns_default_when_no_match_exists() {
        assert_eq!(Solution::two_sum(vec![1, 2, 3], 10), vec![0, 0]);
    }
}
