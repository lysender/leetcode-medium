use leetcode_medium::ListNode;

fn main() {
    Solution::add_two_numbers(None, None);
}

struct Solution {}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        fn create_list(items: Vec<i32>) -> Option<Box<ListNode>> {
            let mut prev: Option<Box<ListNode>> = None;

            for i in items.iter().rev() {
                let node = ListNode {
                    val: *i,
                    next: prev,
                };
                prev = Some(Box::new(node));
            }
            prev
        }

        fn list_to_vec(head: &Option<Box<ListNode>>) -> Vec<i32> {
            let mut items: Vec<i32> = Vec::new();
            let mut current = head;

            while let Some(node) = current {
                items.push(node.val);
                current = &node.next;
            }
            items
        }

        // Convert lists to arrays
        let l1_digits = list_to_vec(&l1);
        let l2_digits = list_to_vec(&l2);
        let max_len = l1_digits.len().max(l2_digits.len());

        // Add digits by digits
        let mut sum_digits: Vec<i32> = vec![0; max_len];
        let mut carry: i32 = 0;

        for i in 0..max_len {
            let d1 = l1_digits.get(i).unwrap_or(&0);
            let d2 = l2_digits.get(i).unwrap_or(&0);
            let sum = d1 + d2 + carry;
            if sum >= 10 {
                sum_digits[i] = sum - 10;
                carry = 1;
            } else {
                carry = 0;
                sum_digits[i] = sum;
            }
        }

        if carry > 0 {
            sum_digits.push(carry);
        }
        create_list(sum_digits)
    }
}

#[cfg(test)]
mod tests {
    use leetcode_medium::create_list;

    use super::*;

    #[test]
    fn test_add_two_numbers_1() {
        let l1 = create_list(vec![2, 4, 3]);
        let l2 = create_list(vec![5, 6, 4]);
        let expected = create_list(vec![7, 0, 8]);
        let result = Solution::add_two_numbers(l1, l2);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_add_two_numbers_2() {
        let l1 = create_list(vec![0]);
        let l2 = create_list(vec![0]);
        let expected = create_list(vec![0]);
        let result = Solution::add_two_numbers(l1, l2);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_add_two_numbers_3() {
        let l1 = create_list(vec![9]);
        let l2 = create_list(vec![1, 9, 9, 9, 9, 9, 9, 9, 9, 9]);
        let expected = create_list(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]);
        let result = Solution::add_two_numbers(l1, l2);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_add_two_numbers_4() {
        let l1 = create_list(vec![9, 9, 9, 9, 9, 9, 9]);
        let l2 = create_list(vec![9, 9, 9, 9]);
        let expected = create_list(vec![8, 9, 9, 9, 0, 0, 0, 1]);
        let result = Solution::add_two_numbers(l1, l2);
        assert_eq!(result, expected);
    }
}
