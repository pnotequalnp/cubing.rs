pub const fn binomial(n: usize, k: usize) -> usize {
    if k > n {
        return 0;
    };

    let k = if k > n - k { n - k } else { k };

    let mut t = 1;

    let mut x = 0;
    while x < k {
        t *= n - x;
        t /= x + 1;

        x += 1;
    }

    t
}

pub const fn factorial(x: usize) -> usize {
    if x < 2 {
        return 1;
    }
    let mut t = x;
    let mut x = x - 1;

    while x > 1 {
        t *= x;
        x -= 1;
    }

    t
}

pub const fn power(base: u8, exponent: usize) -> usize {
    let mut t = base as usize;
    let mut y = exponent;
    while y > 1 {
        t *= base as usize;
        y -= 1;
    }
    t
}

pub const fn count<const N: usize>() -> [usize; N] {
    let mut xs = [0; N];

    let mut ix = 0;
    while ix < N {
        xs[ix] = ix;
        ix += 1;
    }

    xs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn binomials() {
        assert_eq!(495, binomial(12, 4));
        assert_eq!(495, binomial(12, 8));
        assert_eq!(1, binomial(16, 0));
        assert_eq!(13, binomial(13, 1));
        assert_eq!(13, binomial(13, 12));
    }

    #[test]
    pub fn factorials() {
        assert_eq!(1, factorial(0));
        assert_eq!(1, factorial(1));
        assert_eq!(40320, factorial(8));
    }

    #[test]
    pub fn powers() {
        assert_eq!(65536, power(2, 16));
        assert_eq!(8, power(2, 3));
        assert_eq!(2187, power(3, 7));
    }
}
