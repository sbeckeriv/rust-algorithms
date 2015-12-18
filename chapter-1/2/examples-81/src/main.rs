mod examples;

fn main() {
    println!("bob {}", examples::is_palindrome("bob"));
    println!("bobs {}", examples::is_palindrome("bobs"));
    println!("sorted {}", examples::is_sorted("abs"));
    println!("not sorted {}", examples::is_sorted("asb"));
}
