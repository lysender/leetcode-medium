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

    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn find_z(list: &Vec<i32>, x: usize, y: usize, start: usize, z_val: i32) -> Option<usize> {
            let mut low: i32 = start as i32;
            let mut high: i32 = list.len() as i32 - 1;

            while low <= high {
                let mid = low + ((high - low) / 2);
                let val = list[mid as usize];
                if val == z_val {
                    if mid as usize != x && mid as usize != y {
                        // Great, not duplicate
                        return Some(mid as usize);
                    } else {
                        // Find another index down
                        if (mid - 1) < 0 {
                            return None;
                        }

                        let mut z_index = mid as usize - 1;
                        while list[z_index] == z_val {
                            if z_index != x && z_index != y {
                                // Found it among duplicate values
                                return Some(z_index);
                            }
                            if z_index == 0 {
                                // We are out of bounds, try going up next
                                break;
                            }
                            z_index -= 1;
                        }

                        // Still not found? Try going up among duplicates
                        if (mid + 1) > (list.len() as i32 - 1) {
                            return None;
                        }
                        let mut z_index = mid as usize + 1;
                        while list[z_index] == z_val {
                            if z_index != x && z_index != y {
                                // Found it amount duplicates
                                return Some(z_index);
                            }
                            if z_index >= list.len() - 1 {
                                return None;
                            }
                            z_index += 1;
                        }

                        return None;
                    }
                } else if val > z_val {
                    // Go lower
                    high = mid - 1;
                } else {
                    // Go higher
                    low = mid + 1;
                }
            }

            None
        }

        let mut result: Vec<Vec<i32>> = Vec::new();

        // Sort list for faster search later on
        nums.sort();

        for x in 0..nums.len() {
            for y in x + 1..nums.len() {
                let z_val = (nums[x] + nums[y]) * -1;
                let z_result = find_z(&nums, x, y, y + 1, z_val);
                if let Some(z) = z_result {
                    let mut temp: Vec<i32> = vec![nums[x], nums[y], nums[z]];
                    temp.sort();
                    if !result.contains(&temp) {
                        result.push(temp);
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

    fn find_x(list: &Vec<i32>, x: i32) -> Option<usize> {
        let mut low: i32 = 0;
        let mut high: i32 = list.len() as i32 - 1;

        while low <= high {
            let mid = low + ((high - low) / 2);
            if list[mid as usize] == x {
                return Some(mid as usize);
            } else if list[mid as usize] > x {
                // Go lower
                high = mid - 1;
            } else {
                // Go higher
                low = mid + 1;
            }
        }

        None
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
    fn test_three_sum_data2() {
        let mut source: Vec<i32> = vec![
            3, -2, 0, 9, -10, 6, -5, -3, -5, -3, 9, -3, 4, 4, -6, -1, 8, 9, -2, -6, 5, -8, 6,
        ];
        source.sort();

        // Should find all elements
        for i in source.iter() {
            assert_eq!(find_x(&source, *i).is_some(), true);
        }

        // Should not find these
        assert_eq!(find_x(&source, -99).is_some(), false);
        assert_eq!(find_x(&source, 99).is_some(), false);

        let result: Vec<Vec<i32>> = Solution::three_sum(vec![
            3, -2, 0, 9, -10, 6, -5, -3, -5, -3, 9, -3, 4, 4, -6, -1, 8, 9, -2, -6, 5, -8, 6,
        ]);

        println!("{:?}", source);
        println!("{:?}", result);
        assert_eq!(
            is_in_array(
                result,
                vec![
                    vec![-10, 4, 6],
                    vec![-8, -1, 9],
                    vec![-8, 0, 8],
                    vec![-8, 3, 5],
                    vec![-8, 4, 4],
                    vec![-6, -3, 9],
                    vec![-6, -2, 8],
                    vec![-6, 0, 6],
                    vec![-5, -3, 8],
                    vec![-5, -1, 6],
                    vec![-5, 0, 5],
                    vec![-3, -3, 6],
                    vec![-3, -2, 5],
                    vec![-3, -1, 4],
                    vec![-3, 0, 3],
                    vec![-2, -2, 4],
                    vec![-2, -1, 3]
                ]
            ),
            true
        );
    }

    #[test]
    fn test_three_sum_data3() {
        let mut source = vec![-2, 0, 1, 1, 2];
        source.sort();

        let result = Solution::three_sum(vec![-2, 0, 1, 1, 2]);
        println!("{:?}", source);
        println!("{:?}", result);
        assert_eq!(
            is_in_array(
                Solution::three_sum(vec![-2, 0, 1, 1, 2]),
                vec![vec![-2, 0, 2], vec![-2, 1, 1]]
            ),
            true
        );
    }

    #[test]
    fn test_max_area() {
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
        assert_eq!(Solution::max_area(vec![1, 1]), 1);
        assert_eq!(Solution::max_area(vec![2, 3, 10, 5, 7, 8, 9]), 36);
        assert_eq!(Solution::max_area(vec![2, 3, 4, 5, 18, 17, 6]), 17);
    }
}
