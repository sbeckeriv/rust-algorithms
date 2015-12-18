pub fn is_palindrome(s: &str) -> bool {
    let string_iter: Vec<char> = s.chars().collect();
    let s_len = s.len() - 1;
    for i in 0..(s_len / 2) + 1 {
        if string_iter[i] != string_iter[s_len - i] {
            return false;
        }
    }
    true
}
use std::cmp::Ordering;
pub fn is_sorted(s: &str) -> bool {
    let string_iter: Vec<char> = s.chars().collect();
    let s_len = s.len() - 1;
    for i in 0..s_len {
        if Ordering::Less != string_iter[i].cmp(&string_iter[i + 1]) {
            return false;
        }
    }
    true
}
