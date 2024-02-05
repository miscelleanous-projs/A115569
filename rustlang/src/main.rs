use std::collections::HashMap;

// OEIS-A115569
// https://oeis.org/A115569

// Definition:
// Lynch-Bell numbers are numbers n such that the digits are all different (and do not include 0) and n is divisible by each of its individual digits.

fn is_a115569(num: u32) -> bool {
    let mut digit_count = HashMap::new();

    num.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .all(|d| {
            *digit_count.entry(d).or_insert(0) += 1;
            d != 0 && digit_count[&d] <= 1 && num % d == 0
        })
}

fn format_array(numbers: &[u32]) -> String {
    let formatted_numbers: Vec<String> = numbers.iter().map(|&x| x.to_string()).collect();
    format!("[{}]", formatted_numbers.join(", "))
}

fn main() {
    // No "Closed form" for straightforward approach so using Predicate
    // Reproducing... https://oeis.org/A115569/list
    let lynch_bell_numbers: Vec<u32> = (1..=1900).filter(|&x| is_a115569(x)).collect();
    println!("{}", format_array(&lynch_bell_numbers));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_a115569() {
        // no 0 in digits
        assert_eq!(is_a115569(0), false);
        assert_eq!(is_a115569(10), false);
        assert_eq!(is_a115569(123450789), false);

        // duplicate digit(s)
        assert_eq!(is_a115569(11), false);
        assert_eq!(is_a115569(999), false);
        assert_eq!(is_a115569(5555), false);

        // Some Lynch-Bell numbers
        assert_eq!(is_a115569(1), true);
        assert_eq!(is_a115569(36), true);
        assert_eq!(is_a115569(315), true);
        assert_eq!(is_a115569(3195), true);
        assert_eq!(is_a115569(367248), true);
        assert_eq!(is_a115569(9718632), true);
    }
}
