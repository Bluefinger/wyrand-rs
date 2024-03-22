/// Calculate ab (mod m)
///
/// Decompose into sum of powers of 2 (mod m)
#[inline]
const fn mul_mod(a: u64, b: u64, modulo: u64) -> u64 {
    ((a as u128 * b as u128) % modulo as u128) as u64
}

/// Calculate a^b (mod m)
///
/// Decomposes into product of squares (mod m)
#[inline]
const fn pow_mod(mut base: u64, mut exponent: u64, modulo: u64) -> u64 {
    let mut result: u64 = 1;

    while exponent > 0 {
        if exponent & 1 == 1 {
            result = mul_mod(result, base, modulo);
        }
        exponent >>= 1;
        if exponent > 0 {
            base = mul_mod(base, base, modulo);
        }
    }

    result
}

/// Strong Probable Primality test of the number `n` to the base `a`
#[inline]
const fn primality_test(n: u64, witness: u64, mut d: u64) -> bool {
    let mut b = pow_mod(witness, d, n);
    let n_minus = n - 1;

    if b == 1 || b == n_minus {
        return true;
    }

    while d != n_minus {
        b = mul_mod(b, b, n);
        d *= 2;

        if b <= 1 {
            return false;
        }
        if b == n_minus {
            return true;
        }
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

    let mut d = n - 1;
    while d % 2 == 0 {
        d >>= 1; // Same as dividing by two
    }

    primality_test(n, 2, d)
        || n < 2047
        || primality_test(n, 3, d)
        || primality_test(n, 5, d)
        || primality_test(n, 7, d)
        || primality_test(n, 11, d)
        || primality_test(n, 13, d)
        || primality_test(n, 17, d)
        || primality_test(n, 19, d)
        || primality_test(n, 23, d)
        || primality_test(n, 29, d)
        || primality_test(n, 31, d)
        || primality_test(n, 37, d)
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
