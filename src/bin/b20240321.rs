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

        let mut area: i32 = 0;

        for x in 0..(height.len() - 1) {
            for y in 1..height.len() {
                let new_area = compute_area(&height, x, y);
                if new_area > area {
                    area = new_area;
                }
            }
        }

        area
    }

    pub fn max_area_op(height: Vec<i32>) -> i32 {
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
        let mut max_l: usize = l;
        let mut max_r: usize = r;
        let mut area: i32 = compute_area(&height, l, r);

        while l < r {
            // Move left forward
            l += 1;
            let mut new_area = compute_area(&height, l, max_r);
            if new_area > area {
                max_l = l;
                area = new_area;
            }

            // Move right backward
            r -= 1;
            new_area = compute_area(&height, max_l, r);
            if new_area > area {
                max_r = r;
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
