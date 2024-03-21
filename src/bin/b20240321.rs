fn main() {
    Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]);
    Solution::three_sum(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]);
}

struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut l: usize = 0;
        let mut r: usize = height.len() - 1;
        let mut area: i32 = 0;

        while l < r {
            let l_val = height[l];
            let r_val = height[r];
            let current_area = l_val.min(r_val) * (r - l) as i32;
            if current_area > area {
                area = current_area;
            }

            // Whichever is lower must move inward
            if height[l] < height[r] {
                l += 1;
            } else {
                r -= 1;
            }
        }
        area
    }

    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                for k in j + 1..nums.len() {
                    if nums[i] + nums[j] + nums[k] == 0 {
                        let mut temp: Vec<i32> = vec![nums[i], nums[j], nums[k]];
                        temp.sort();
                        if !result.contains(&temp) {
                            result.push(temp);
                        }
                    }
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_in_array(source: Vec<Vec<i32>>, target: Vec<Vec<i32>>) -> bool {
        for i in target {
            if !source.contains(&i) {
                return false;
            }
        }
        true
    }

    #[test]
    fn test_three_sum() {
        assert_eq!(
            is_in_array(
                Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
                vec![vec![-1, -1, 2], vec![-1, 0, 1]]
            ),
            true
        );
        assert_eq!(
            is_in_array(Solution::three_sum(vec![0, 0, 0]), vec![vec![0, 0, 0]]),
            true
        );
        let empty: Vec<Vec<i32>> = vec![];
        assert_eq!(is_in_array(Solution::three_sum(vec![0, 1, 1]), empty), true);
    }

    #[test]
    fn test_max_area() {
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
        assert_eq!(Solution::max_area(vec![1, 1]), 1);
        assert_eq!(Solution::max_area(vec![2, 3, 10, 5, 7, 8, 9]), 36);
        assert_eq!(Solution::max_area(vec![2, 3, 4, 5, 18, 17, 6]), 17);
    }
}
