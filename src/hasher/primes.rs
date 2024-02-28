/// Calculate ab (mod m)
///
/// Decompose into sum of powers of 2 (mod m)
#[inline]
const fn mul_mod(mut a: u64, mut b: u64, m: u64) -> u64 {
    let mut r: u64 = 0;

    while b > 0 {
        if b & 1 == 1 {
            let mut r2 = r.wrapping_add(a);
            if r2 < r {
                r2 = r2.wrapping_sub(m);
            }
            r = r2 % m;
        }
        b >>= 1;
        if b > 0 {
            let mut a2 = a.wrapping_add(a);
            if a2 < a {
                a2 = a2.wrapping_sub(m);
            }
            a = a2 % m;
        }
    }

    r
}

/// Calculate a^b (mod m)
///
/// Decomposes into product of squares (mod m)
#[inline]
const fn pow_mod(mut a: u64, mut b: u64, m: u64) -> u64 {
    let mut r: u64 = 1;

    while b > 0 {
        if b & 1 == 1 {
            r = mul_mod(r, a, m);
        }
        b >>= 1;
        if b > 0 {
            a = mul_mod(a, a, m);
        }
    }

    r
}

/// Strong Probable Primality test of the number `n` to the base `a`
#[inline]
const fn sprp(n: u64, a: u64) -> bool {
    let mut d = n - 1;
    let mut s: u8 = 0;

    while (d & 0xff) == 0 {
        d >>= 8;
        s += 8;
    }

    if (d & 0xf) == 0 {
        d >>= 4;
        s += 4;
    }

    if (d & 0x3) == 0 {
        d >>= 2;
        s += 2;
    }

    if (d & 0x1) == 0 {
        d >>= 1;
        s += 1;
    }

    let mut b = pow_mod(a, d, n);

    if b == 1 || b == (n - 1) {
        return true;
    }

    let mut r: u8 = 1;

    while r < s {
        b = mul_mod(b, b, n);

        if b <= 1 {
            return false;
        }
        if b == (n - 1) {
            return true;
        }

        r += 1;
    }

    false
}

/// Checks `n` for whether it is a prime number. Calculates for all primes
/// within the `u64` space.
#[inline]
pub(super) const fn is_prime(n: u64) -> bool {
    // 0 & 1 are not primes
    if n < 2 {
        return false;
    }
    // 2 & 3 are prime numbers
    if n < 4 {
        return true;
    }
    // Even numbers after 2 are not prime numbers
    if (n & 1) == 0 {
        return false;
    }

    sprp(n, 2)
        || n < 2047
        || sprp(n, 3)
        || sprp(n, 5)
        || sprp(n, 7)
        || sprp(n, 11)
        || sprp(n, 13)
        || sprp(n, 17)
        || sprp(n, 19)
        || sprp(n, 23)
        || sprp(n, 29)
        || sprp(n, 31)
        || sprp(n, 37)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_prime_numbers_correctly() {
        let test_cases: [(u64, bool); 8] = [
            (1, false),
            (2, true),
            (8, false),
            (1723, true),
            (3191, true),
            (10001, false),
            (87178291199, true),
            (u64::MAX, false),
        ];

        test_cases.into_iter().for_each(|(n, expected)| {
            assert_eq!(is_prime(n), expected, "Failed prime check for: {}", n);
        });
    }
}
