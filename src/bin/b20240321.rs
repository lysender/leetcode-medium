use leetcode_medium::ListNode;

fn main() {
    Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]);
    Solution::three_sum(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]);
    Solution::letter_combinations("".to_string());
    Solution::remove_nth_from_end(None, 1);
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
        let mut result: Vec<Vec<i32>> = Vec::new();
        nums.sort();

        // 0 = x + y + z
        // Will do a first loop for x and do an inner loop with low/high pair for y and z
        for x in 0..nums.len() - 2 {
            // If we move forward and found same number, we probably already
            // solved that combination from the previous iteration
            // so no need to repeat it as the outcome is the same
            if x > 0 && nums[x] == nums[x - 1] {
                continue;
            }

            // Find the 2 numbers so that 0 = nums[x] + nums[y] + nums[z]
            let mut y = x + 1;
            let mut z = nums.len() - 1;

            while y < z {
                let sum = nums[x] + nums[y] + nums[z];
                if sum > 0 {
                    // Too high, we need to lower our upper bound
                    z -= 1;
                } else if sum < 0 {
                    // To low, we need to increase our lower bound
                    y += 1;
                } else {
                    // Must be equal, we found our pair
                    result.push(vec![nums[x], nums[y], nums[z]]);

                    // Move our lower bound higher
                    // We may be able to use the opposite side but we will
                    // just increase our lower bound for now
                    y += 1;

                    // Skip a couple of items if the next items are same values
                    // as we don't support duplicate items
                    while nums[y] == nums[y - 1] && y < z {
                        y += 1;
                    }
                }
            }
        }

        result
    }

    pub fn letter_combinations(digits: String) -> Vec<String> {
        // Map 2-9 to 0-7
        let map: Vec<&str> = vec!["abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];

        // Collect the selected keys
        let key_maps: Vec<Vec<u8>> = digits
            .chars()
            .map(|i| {
                let key: usize = i.to_string().parse().unwrap();
                map[key - 2].to_string().as_bytes().to_vec()
            })
            .collect();

        // Generate the combinations
        fn gen_combinations(
            map: &Vec<Vec<u8>>,
            combi: &mut Vec<String>,
            prefix: Vec<u8>,
            index: usize,
        ) {
            let group = &map[index];

            if index == map.len() - 1 {
                // End of the line, let's push all combinations
                for i in group.iter() {
                    let mut new_prefix = prefix.clone();
                    new_prefix.push(*i);
                    let str = String::from_utf8_lossy(&new_prefix).to_string();
                    combi.push(str);
                }
            } else {
                // Still need to run all other keys
                for i in group.iter() {
                    let mut new_prefix = prefix.clone();
                    new_prefix.push(*i);

                    gen_combinations(map, combi, new_prefix, index + 1);
                }
            }
        }

        let mut result: Vec<String> = Vec::new();

        if key_maps.len() > 0 {
            gen_combinations(&key_maps, &mut result, vec![], 0);
        }

        result
    }

    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        // Use a naive solution
        // Get the length of the list
        // Traverse again up to target - 1, remove target and point next to the next item
        let mut length: i32 = 0;
        let mut node: &Option<Box<ListNode>> = &head;
        while let Some(node_val) = node {
            length += 1;
            node = &node_val.next;
        }

        // Create a new list to contain the result with node removed from it
        let mut result = Some(Box::new(ListNode::new(0)));
        let mut current = result.as_mut();
        let mut node = &head;
        let mut x: i32 = 1;

        while let Some(node_val) = node {
            if (length - x + 1) != n {
                current.as_mut().unwrap().next = Some(Box::new(ListNode::new(node_val.val)));
                current = current.unwrap().next.as_mut();
            }
            x += 1;
            node = &node_val.next;
        }

        result.unwrap().next
    }
}

#[cfg(test)]
mod tests {
    use leetcode_medium::{create_list, list_to_vec};

    use super::*;

    #[test]
    fn test_removed_nth_from_end_1() {
        let input = create_list(vec![1, 2, 3, 4, 5]);
        let expected: Vec<i32> = vec![1, 2, 3, 5];
        let output = Solution::remove_nth_from_end(input, 2);
        assert_eq!(list_to_vec(output), expected);
    }

    #[test]
    fn test_removed_nth_from_end_2() {
        let input = create_list(vec![1]);
        let expected: Vec<i32> = vec![];
        let output = Solution::remove_nth_from_end(input, 1);
        assert_eq!(list_to_vec(output), expected);
    }

    #[test]
    fn test_removed_nth_from_end_3() {
        let input = create_list(vec![1, 2]);
        let expected: Vec<i32> = vec![1];
        let output = Solution::remove_nth_from_end(input, 1);
        assert_eq!(list_to_vec(output), expected);
    }

    #[test]
    fn test_letter_combinations_1() {
        let input = String::from("23");
        let expected: Vec<String> = vec![
            "ad".to_string(),
            "ae".to_string(),
            "af".to_string(),
            "bd".to_string(),
            "be".to_string(),
            "bf".to_string(),
            "cd".to_string(),
            "ce".to_string(),
            "cf".to_string(),
        ];
        assert_eq!(Solution::letter_combinations(input), expected);
    }

    #[test]
    fn test_letter_combinations_2() {
        let input = String::from("");
        let expected: Vec<String> = Vec::new();
        assert_eq!(Solution::letter_combinations(input), expected);
    }

    #[test]
    fn test_letter_combinations_3() {
        let input = String::from("2");
        let expected: Vec<String> = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        assert_eq!(Solution::letter_combinations(input), expected);
    }

    fn is_in_array(source: Vec<Vec<i32>>, target: Vec<Vec<i32>>) -> bool {
        if source.len() != target.len() {
            return false;
        }
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

        assert_eq!(
            is_in_array(Solution::three_sum(vec![0, 0, 0, 0]), vec![vec![0, 0, 0]]),
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
