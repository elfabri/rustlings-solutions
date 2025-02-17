fn bigger(a: i32, b: i32) -> i32 {
    // function to return the bigger number!
    // If both numbers are equal, any of them can be returned.
    // Do not use:
    // - another function call
    // - additional variables
    if a > b {
        a
    } else {
        b
    }
}

fn main() {
    // You can optionally experiment here.
    println!("3 or 5: {}", bigger(3, 5));
    println!("4 or 9: {}", bigger(4, 9));
    println!("9 or 5: {}", bigger(9, 5));
    println!("105234 or 29340: {}", bigger(29_340, 105_234));
    println!("7 or 7: {}", bigger(3, 5));
}

// Don't mind this for now :)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }

    #[test]
    fn equal_numbers() {
        assert_eq!(42, bigger(42, 42));
    }
}
