use std::str::FromStr;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Debug, PartialOrd, Ord)]
pub struct MacAddr {
    octets: [u8; 6],
}

impl MacAddr {

    #[inline]
    pub const fn new(a: u8, b: u8, c: u8, d: u8, e: u8, f: u8) -> Self {
        MacAddr {
            octets: [a, b, c, d, e, f],
        }
    }

    #[inline]
    pub const fn from_octets(octets: [u8; 6]) -> Self {
        MacAddr { octets }
    }

    #[inline]
    pub const fn octets(&self) -> [u8; 6] {
        self.octets
    }


}

impl std::fmt::Display for MacAddr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.octets
                .iter()
                .map(|x| format!("{:X}", x))
                .collect::<Vec<String>>()
                .join(":")
        )
    }
}

impl FromStr for MacAddr {
    type Err = MacAddrParseError;

    fn from_str(mac_str: &str) -> Result<Self, MacAddrParseError> {
        let mut octets = [0u8; 6];

        if mac_str.len() > 17 {
            return Err(MacAddrParseError);
        }

        for (idx, _) in mac_str.char_indices().step_by(3) {
            let hex_digit = match mac_str[idx..].chars().next() {
                Some(c) => c.to_digit(16).ok_or(MacAddrParseError)?,
                None => return Err(MacAddrParseError),
            };
            octets[idx / 3] = hex_digit.try_into().map_err(|_| MacAddrParseError)?;
        }

        Ok(octets.into())
    }
}

impl From<[u8; 6]> for MacAddr {
    fn from(octets: [u8; 6]) -> Self {
        MacAddr::from_octets(octets)
    }
}

#[derive(Debug)]
pub struct MacAddrParseError;

impl From<std::io::Error> for MacAddrParseError {
    fn from(_: std::io::Error) -> Self {
        MacAddrParseError
    }
}
