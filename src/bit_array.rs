use std::ops::BitOr;

#[derive(Clone)]
pub struct BitArray(
    Box<[u8]>,
    #[cfg(debug_assertions)]
    usize
);


impl BitArray {
    pub fn new(n: usize) -> Self {
        let len = n / 8 + (n % 8 != 0) as usize;
        let bit_array = vec![0; len];
        debug_assert_eq!(bit_array.capacity(), len);
        Self(
            bit_array.into(),
            #[cfg(debug_assertions)] n,
        )
    }

    fn debug_check_bounds(&self, i: usize) {
        #[cfg(debug_assertions)]
        assert!(i < self.1);
    }

    pub fn get(&self, i: usize) -> bool {
        self.debug_check_bounds(i);
        (self.0[i / 8] >> (i % 8) & 1) != 0
    }

    pub fn set(&mut self, i: usize, has: bool) {
        self.debug_check_bounds(i);
        let x = &mut self.0[i / 8];
        let pos = i % 8;
        let nth_bit_set = (*x >> pos) & 1;
        let toggle = nth_bit_set ^ (has as u8);
        *x ^= toggle << pos;
    }

    pub fn add(&mut self, i: usize) {
        self.debug_check_bounds(i);
        self.0[i / 8] |= 1 << (i % 8)
    }

    pub fn add_if(&mut self, i: usize, cond: bool) {
        self.debug_check_bounds(i);
        self.0[i / 8] |= (cond as u8) << (i % 8)
    }

    pub fn rm(&mut self, i: usize) {
        self.debug_check_bounds(i);
        self.0[i / 8] &= !(1 << (i % 8));
    }

    pub fn rm_if(&mut self, i: usize, cond: bool) {
        self.debug_check_bounds(i);
        self.0[i / 8] &= !((cond as u8) << (i % 8));
    }
}

impl BitOr for &BitArray {
    type Output = BitArray;

    fn bitor(self, rhs: Self) -> Self::Output {
        let (shortest, longest) = if self.0.len() < rhs.0.len() {
            (self, rhs)
        } else {
            (rhs, self)
        };
        let mut result = longest.clone();

        result.0.iter_mut()
            .zip(shortest.0.iter())
            .for_each(|(x, b)| *x |= b);

        result
    }
}

impl<const N: usize> From<[u8; N]> for BitArray {
    fn from(array: [u8; N]) -> Self {
        Self(
            Box::new(array),
            #[cfg(debug_assertions)] (8 * N),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::BitArray;

    #[test]
    fn new() {
        let bs = BitArray::new(10);
        assert_eq!(bs.1, 10);
        assert_eq!(bs.0.len(), 2);
    }

    #[test]
    #[should_panic]
    fn out_of_bounds() {
        let bs = BitArray::new(18);
        bs.get(18);
    }

    #[test]
    fn has() {
        let bs = BitArray::from([0b_1010_1110]);

        assert_eq!(bs.get(0), false);
        assert_eq!(bs.get(1), true);
        assert_eq!(bs.get(2), true);
        assert_eq!(bs.get(3), true);
        assert_eq!(bs.get(4), false);
        assert_eq!(bs.get(5), true);
        assert_eq!(bs.get(6), false);
        assert_eq!(bs.get(7), true);
    }

    #[test]
    fn add() {
        let mut bs = BitArray::from([0b_1010_1110]);

        // Check does change
        assert_eq!(bs.get(0), false);
        bs.add(0);
        assert_eq!(bs.get(0), true);

        // Check doesn't change
        bs.add(0);
        assert_eq!(bs.get(1), true);
    }

    #[test]
    fn rm() {
        let mut bs = BitArray::from([0b_1010_1110]);

        // Check doesn't change
        assert_eq!(bs.get(0), false);
        bs.rm(0);
        assert_eq!(bs.get(0), false);

        // Check does change
        assert_eq!(bs.get(1), true);
        bs.rm(1);
        assert_eq!(bs.get(1), false);
    }
}
