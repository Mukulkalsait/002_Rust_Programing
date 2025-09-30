// @leet start

impl Solution {
    pub fn is_valid(s: String) -> bool {
        if s.len() % 2 != 0 {
            false
        } else {
            let mut stack: Vec<char> = Vec::new();
            for a in s.chars() {
                if a = '{' || a = '(' || a = '[' {
                    stack.push(a);
                } else if a = '}' || a = ')' || a = ']' {
                }
            }
            false
        }
    }
}
// @leet end

