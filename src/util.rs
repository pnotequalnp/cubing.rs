pub const fn factorial(x: u8) -> usize {
    if x < 2 { return 1; }
    let mut t = x as usize;
    let mut x = x as usize - 1;

    while x > 1 {
        t *= x;
        x -= 1;
    };

    t
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn facts() {
        assert_eq!(1, factorial(0));
        assert_eq!(1, factorial(1));
        assert_eq!(40320, factorial(8));
    }
}
