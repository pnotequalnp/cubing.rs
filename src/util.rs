pub const fn factorial(x: usize) -> usize {
    if x < 2 { return 1; }
    let mut t = x;
    let mut x = x - 1;

    while x > 1 {
        t *= x;
        x -= 1;
    };

    t
}

pub const fn power(base: u8, exponent: usize) -> usize {
    let mut t = base as usize;
    let mut y = exponent;
    while y > 1 {
        t *= base as usize;
        y -= 1;
    };
    t
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn exponents() {
        assert_eq!(65536, power(2, 16));
        assert_eq!(8, power(2, 3));
        assert_eq!(2187, power(3, 7));
    }

    #[test]
    pub fn factorialss() {
        assert_eq!(1, factorial(0));
        assert_eq!(1, factorial(1));
        assert_eq!(40320, factorial(8));
    }
}
