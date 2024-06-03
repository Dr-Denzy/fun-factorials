fn factorial_recursion(n: u32) -> u32 {
    if n == 0 {
        1
    } else {
        n * factorial_recursion(n - 1)
    }
}

fn factorial_while(mut n: u32) -> u32 {
    let mut result = 1;

    while n > 0 {
        result *= n;
        n -= 1;
    }
    result
}

fn factorial_for(n: u32) -> u32 {
    let mut result = 1;

    for i in 1..=n {
        result *= i;
    }
    result
}

fn factorial_loop(mut n: u32) -> u32 {
    let mut result = 1;
    loop {
        if n == 0 {
            break;
        }
        result *= n;
        n -= 1;
    }
    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_recursion() {
        assert_eq!(factorial_recursion(0), 1);
    }

    #[test]
    fn second_recursion() {
        assert_eq!(factorial_recursion(1), 1);
    }

    #[test]
    fn third_recursion() {
        assert_eq!(factorial_recursion(2), 2);
    }

    #[test]
    fn fourth_recursion() { assert_eq!(factorial_recursion(5), 120); }

    #[test]
    fn first_while() {
        assert_eq!(factorial_while(0), 1);
    }

    #[test]
    fn second_while() {
        assert_eq!(factorial_while(1), 1);
    }

    #[test]
    fn third_while() {
        assert_eq!(factorial_while(2), 2);
    }

    #[test]
    fn fourth_while() { assert_eq!(factorial_while(5), 120); }

    #[test]
    fn first_for() {
        assert_eq!(factorial_for(0), 1);
    }

    #[test]
    fn second_for() { assert_eq!(factorial_for(1), 1); }

    #[test]
    fn third_for() {
        assert_eq!(factorial_for(2), 2);
    }

    #[test]
    fn fourth_for() { assert_eq!(factorial_for(5), 120); }

    #[test]
    fn first_loop() {
        assert_eq!(factorial_loop(0), 1);
    }

    #[test]
    fn second_loop() { assert_eq!(factorial_loop(1), 1); }

    #[test]
    fn third_loop() {
        assert_eq!(factorial_loop(2), 2);
    }

    #[test]
    fn fourth_loop() { assert_eq!(factorial_loop(5), 120); }
}
