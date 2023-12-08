#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct MacAddr {
    octets: [u8; 6],
}
impl MacAddr {
    pub const fn new(a: u8, b: u8, c: u8, d: u8, e: u8, f: u8) -> Self {
        MacAddr {
            octets: [a, b, c, d, e, f],
        }
    }

    #[inline]
    pub const fn octets(&self) -> [u8; 6] {
        self.octets
    }
}
