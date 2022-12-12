//! Pseudo random number generator based on the XORSHIFT64 algorithm.
//!
//! See [Wiki Article](https://en.wikipedia.org/wiki/Xorshift) for more information.
pub struct Random(u64);

impl Random {
    pub fn new() -> Self {
        Self(0xFFAAB58C5833FE89)
    }

    /// Returns a random unsigned 64-bit integer.
    pub fn next_u64(&mut self) -> u64 {
        let mut number = self.0;

        number ^= number << 13;
        number ^= number >> 17;
        number ^= number << 5;

        self.0 = number;

        number
    }
}
