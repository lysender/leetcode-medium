fn main() {
    Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]);
}

struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        if height.len() < 2 {
            return 0;
        }

        fn compute_area(config: &Vec<i32>, left: usize, right: usize) -> i32 {
            let l_val = config[left];
            let r_val = config[right];
            l_val.min(r_val) * (right as i32 - left as i32)
        }

        let mut l: usize = 0;
        let mut r: usize = height.len() - 1;
        let mut area: i32 = compute_area(&height, l, r);

        while l < r {
            // Whichever is lower must move inward
            if height[l] < height[r] {
                l += 1;
            } else {
                r -= 1;
            }

            let new_area = compute_area(&height, l, r);
            if new_area > area {
                area = new_area;
            }
        }
        area
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_area() {
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
        assert_eq!(Solution::max_area(vec![1, 1]), 1);
        assert_eq!(Solution::max_area(vec![2, 3, 10, 5, 7, 8, 9]), 36);
        assert_eq!(Solution::max_area(vec![2, 3, 4, 5, 18, 17, 6]), 17);
    }
}
