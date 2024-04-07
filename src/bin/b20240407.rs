fn main() {
    Solution::generate_parenthesis(1);
}

struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        fn gen_paren(prefix: String, opening: i32, closing: i32, max: i32) -> Vec<String> {
            let mut result = Vec::new();

            // Generated a valid parenthesis pattern
            if opening == closing && closing == max {
                result.push(prefix);
                return result;
            }

            // Add more parenthesis
            // Can we still add ( char?
            if opening < max {
                let mut open_prefix = prefix.clone();
                open_prefix.push('(');
                let mut open_result = gen_paren(open_prefix, opening + 1, closing, max);
                result.append(&mut open_result);
            }

            // Can we still add ) char?
            if opening > closing {
                let mut close_prefix = prefix.clone();
                close_prefix.push(')');
                let mut close_result = gen_paren(close_prefix, opening, closing + 1, max);
                result.append(&mut close_result);
            }

            result
        }

        // Given n, we are only expecting n * 2 characters, either ( or )
        // First character is always the ( char so we start there
        gen_paren("(".to_string(), 1, 0, n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_parenthesis() {
        assert_eq!(
            Solution::generate_parenthesis(3),
            vec!["((()))", "(()())", "(())()", "()(())", "()()()"]
        );
        assert_eq!(Solution::generate_parenthesis(1), vec!["()"]);
    }
}
