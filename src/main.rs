fn main() {
    println!("Hello, world!");
}

/// fizzbuzz
/// ---
/// returns “Fizz” if n is divisible by 3
/// returns “Buzz” if n is divisible by 5
/// returns “FizzBuzz” if n is divisible by both 3 and 5
pub fn fizzbuzz(n: usize) -> String {
    match n {
        x if x%(3*5) == 0 => return String::from("FizzBuzz"),
        x if x%3 == 0 => return String::from("Fizz"),
        x if x%5 == 0 => return String::from("Buzz"),
        _ => return n.to_string(),
    }
}

#[test]
fn fizzbuzz_gives_correct_result_for_a_the_first_15_values() {
    let result = ["1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz", "13", "14", "FizzBuzz"];
    for (i, x) in result.iter().enumerate() {
        assert_eq!(fizzbuzz(i+1), x.to_string());
    }
}


