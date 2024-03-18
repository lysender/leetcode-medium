use leetcode_medium::ListNode;

fn main() {
    Solution::add_two_numbers(None, None);
    Solution::add_two_numbers_v2(None, None);
    Solution::length_of_longest_substring(String::from("abcabcbb"));
    Solution::longest_palindrome(String::from("babad"));
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

    pub fn add_two_numbers_v2(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut carry: i32 = 0;

        let mut cur_l1 = &l1;
        let mut cur_l2 = &l2;

        // Create a new linked list for the result
        let mut l3 = Some(Box::new(ListNode::new(0)));
        let mut cur_l3 = l3.as_mut();

        while cur_l1.is_some() || cur_l2.is_some() {
            let mut sum: i32 = carry;
            if let Some(l1_val) = cur_l1 {
                sum += l1_val.val;
                cur_l1 = &l1_val.next;
            }
            if let Some(l2_val) = cur_l2 {
                sum += l2_val.val;
                cur_l2 = &l2_val.next;
            }

            let mut digit: i32 = sum;
            carry = 0;

            if sum >= 10 {
                digit = sum - 10;
                carry = 1;
            }

            // Create a new node
            let node = Some(Box::new(ListNode::new(digit)));
            // Link it to the current node
            cur_l3.as_mut().unwrap().next = node;
            // Make current as the new next node
            cur_l3 = cur_l3.unwrap().next.as_mut();
        }

        if carry > 0 {
            let node = Some(Box::new(ListNode::new(carry)));
            cur_l3.as_mut().unwrap().next = node;
        }

        l3.unwrap().next
    }

    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut longest: usize = 0;
        let mut stack: Vec<u8> = Vec::new();

        for ch in s.as_bytes().iter() {
            if let Some(index) = stack.iter().position(|x| x == ch) {
                // Already exists, remove all elements before the existing index
                // including the duplicate element itself
                stack.splice(0..(index + 1), []);
            }
            stack.push(*ch);
            longest = longest.max(stack.len());
        }
        longest as i32
    }

    pub fn longest_palindrome(s: String) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use leetcode_medium::create_list;

    use super::*;

    #[test]
    fn test_longest_palindrome() {
        assert_eq!(
            Solution::longest_palindrome(String::from("babad")),
            String::from("bab")
        );
        assert_eq!(
            Solution::longest_palindrome(String::from("cbbd")),
            String::from("bb")
        );
    }

    #[test]
    fn test_length_of_longest_substring() {
        assert_eq!(
            Solution::length_of_longest_substring(String::from("abcabcbb")),
            3
        );
        assert_eq!(
            Solution::length_of_longest_substring(String::from("bbbbb")),
            1
        );
        assert_eq!(
            Solution::length_of_longest_substring(String::from("pwwkew")),
            3
        );
    }

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

    #[test]
    fn test_add_two_numbers_5() {
        let l1 = create_list(vec![2, 4, 3]);
        let l2 = create_list(vec![5, 6, 4]);
        let expected = create_list(vec![7, 0, 8]);
        let result = Solution::add_two_numbers_v2(l1, l2);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_add_two_numbers_6() {
        let l1 = create_list(vec![0]);
        let l2 = create_list(vec![0]);
        let expected = create_list(vec![0]);
        let result = Solution::add_two_numbers_v2(l1, l2);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_add_two_numbers_7() {
        let l1 = create_list(vec![9]);
        let l2 = create_list(vec![1, 9, 9, 9, 9, 9, 9, 9, 9, 9]);
        let expected = create_list(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]);
        let result = Solution::add_two_numbers_v2(l1, l2);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_add_two_numbers_8() {
        let l1 = create_list(vec![9, 9, 9, 9, 9, 9, 9]);
        let l2 = create_list(vec![9, 9, 9, 9]);
        let expected = create_list(vec![8, 9, 9, 9, 0, 0, 0, 1]);
        let result = Solution::add_two_numbers_v2(l1, l2);
        assert_eq!(result, expected);
    }
}
