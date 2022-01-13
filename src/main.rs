use memoize::memoize;
use std::io::Write;


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

// I would have wanted to avoid using a library for memoizing and instead used a static hashmap<usize,usize> 
// that I would update continously with the values returned from the fib. 
// But it seems like that is a bit more involved in rust than I am familiar with so I will use the library instead
// 
/// fib
/// ---
/// fib(0) = 0
/// fib(1) = 1
/// fib(n) = fib(n-1) + fib(n-2) for all n >= 0
#[memoize]
pub fn fib(n: usize) -> usize {
    match n {
        0 => return 0,
        1 => return 1,
        _ => return fib(n-1) + fib(n-2)
    }
}

// Found an example of the first 21 Fibonacci numbers here https://en.wikipedia.org/wiki/Fibonacci_number
#[test]
fn fib_gives_correct_result_a_few_values() {
    assert_eq!(fib(0), 0);
    assert_eq!(fib(1), 1);
    assert_eq!(fib(2), 1);
    assert_eq!(fib(3), 2);
    assert_eq!(fib(10), 55);
    assert_eq!(fib(19), 4181);
}


struct Rot13Writer<T>
where
    T: Write,
{
    content: T
}

impl<T> Rot13Writer<T>
where
    T: Write,
{
    pub fn new(inner: T) -> Self {
        return Rot13Writer { content: inner}
    }
}

impl<T> Write for Rot13Writer<T>
where
    T: Write,
{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut pos = 0;
        while pos < buf.len() {
            let bytes_written = self.content.write(&buf[pos..])?;
            pos += bytes_written;
        }

        Ok(pos)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

fn main() {
    let mut content = Vec::<u8>::default();
    let mut buff = Rot13Writer::new(&mut content);
    buff.write(b"Lbh penpxrq zl fhcre qvssvphyg pbqvat punyyratr... pbqr vf ddommNst")
        .unwrap();

    println!(
        "result: {:?}",
        content.iter().map(|x| *x as char).collect::<String>()
    );
}

#[test]
fn test_rot13() {
    main()
}
