fn get_string(x: i32) -> String {
    match (x % 3, x % 5) {
        (0, 0) => format!("FizzBuzz"),
        (0, _) => format!("Fizz"),
        (_, 0) => format!("Buzz"),
        _ => format!("{}", x),
    }
}

fn main() {
    for x in 1..=100 {
        println!("{}", get_string(x));
    }
}

#[cfg(test)]
mod test_fizz_buzz {
    use super::*;

    #[test]
    fn test_fizz() {
        for x in &[3, 6, 27] {
            assert_eq!(get_string(*x), "Fizz")
        }
    }

    #[test]
    fn test_buzz() {
        for x in &[5, 10, 20] {
            assert_eq!(get_string(*x), "Buzz")
        }
    }

    #[test]
    fn test_fizzbuzz() {
        for x in &[15, 30, 60] {
            assert_eq!(get_string(*x), "FizzBuzz")
        }
    }

    #[test]
    fn test_num() {
        for x in &[13, 29, 98] {
            assert_eq!(get_string(*x), format!("{}", x))
        }
    }
}
