pub struct PrimeFactorizer {
    buf: Box<[usize]>,
}

impl PrimeFactorizer {
    pub fn prepare(upper_bound: usize) -> Self {
        let mut buf = vec![0; upper_bound + 1];

        for n in 2..=upper_bound {
            for k in 1.. {
                if n * k > upper_bound {
                    break;
                }
                if buf[n * k] != 0 {
                    continue;
                }
                buf[n * k] = n;
            }
        }

        Self {
            buf: buf.into_boxed_slice(),
        }
    }

    pub fn factorize(&self, n: usize, factors: &mut Vec<usize>) {
        let mut n = n;
        while n > 1 {
            let factor = self.buf[n];
            factors.push(factor);
            n /= factor;
        }
    }
}
