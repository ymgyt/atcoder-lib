use num::PrimInt;

pub fn gcd<T: PrimInt>(a: T, b: T) -> T {
    debug_assert!(!a.is_zero() && !b.is_zero());
    let (mut a, mut b) = (a, b);
    if b > a {
        std::mem::swap(&mut a, &mut b);
    }

    let mut c;
    loop {
        c = a % b;
        if c.is_zero() {
            return b;
        }
        a = b;
        b = c;
    }
}
