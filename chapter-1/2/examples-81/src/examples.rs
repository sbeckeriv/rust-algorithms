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
