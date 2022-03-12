/// A trait representing a sized type which can be converted to and from bytes.
pub trait SizedNumber
where
    Self: Sized,
{
    /// Returns the size of `Self` in bytes.
    ///
    /// If unimplemented, calls [`std::mem::size_of`] [`crate::traits::SizedNumber`] on `Self`.
    ///
    /// # Example
    ///
    /// ```
    /// use seqbytes::traits::SizedNumber;
    ///
    /// assert_eq!(f32::size(), 4);
    /// assert_eq!(u32::size(), 4);
    /// assert_eq!(i32::size(), 4);
    /// assert_eq!(f64::size(), 8);
    /// assert_eq!(u64::size(), 8);
    /// assert_eq!(i64::size(), 8);
    /// assert_eq!(u128::size(), 16);
    /// assert_eq!(i128::size(), 16);
    /// ```
    fn size() -> usize {
        std::mem::size_of::<Self>()
    }

    /// Converts the slice to `Self`. Will return [`None`] if the slice length is not equal to the size of the type.
    ///
    /// # Example
    ///
    /// ```
    /// use seqbytes::traits::*;
    ///
    /// let a = 22.4f64;
    /// let b = a.to_bytes();
    /// let c = f64::from_bytes(&b);
    /// let d = u32::from_bytes(&[2, 4, 6]);
    /// assert_eq!(c, Some(22.4));
    /// assert_eq!(d, None);
    /// ```
    fn from_bytes(bytes: &[u8]) -> Option<Self>;
    /// Converts `self` to equivalent byte representation.
    ///
    /// # Example
    ///
    /// ```
    /// use seqbytes::traits::*;
    ///
    /// let a = 22.4f64;
    /// let b = a.to_bytes();
    /// let c = f64::from_bytes_e(&b, false).unwrap();
    /// assert_eq!(c, 22.4);
    /// ```
    fn to_bytes(&self) -> Vec<u8>;
}

/// A trait representing a sized type which can be converted to and from bytes with a specific endianness.
pub trait EndianNumber: SizedNumber {
    /// Converts the slice to `Self` with the specified endianness. Will return [`None`] if the slice length is not equal to the size of the type.
    fn from_bytes_e(bytes: &[u8], bigendian: bool) -> Option<Self>;
    /// Converts `self` to equivalent byte representation in the specified endianness.
    fn to_bytes_e(&self, bigendian: bool) -> Vec<u8>;
}

impl SizedNumber for u8 {
    fn size() -> usize {
        1 // Size of byte is 1 byte, duhhh
    }

    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() != 1 {
            return None;
        }

        Some(bytes[0])
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![*self]
    }
}
impl EndianNumber for u8 {
    fn from_bytes_e(bytes: &[u8], _bigendian: bool) -> Option<Self> {
        Self::from_bytes(bytes)
    }

    fn to_bytes_e(&self, _bigendian: bool) -> Vec<u8> {
        self.to_bytes()
    }
}

impl SizedNumber for i8 {
    fn size() -> usize {
        1
    }
    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() != 1 {
            return None;
        }

        Some(bytes[0] as i8)
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![(*self) as u8]
    }
}
impl EndianNumber for i8 {
    fn from_bytes_e(bytes: &[u8], _bigendian: bool) -> Option<Self> {
        Self::from_bytes(bytes)
    }

    fn to_bytes_e(&self, _bigendian: bool) -> Vec<u8> {
        self.to_bytes()
    }
}

impl SizedNumber for u16 {
    fn size() -> usize {
        2
    }

    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() != 2 {
            return None;
        }

        Some(Self::from_le_bytes([bytes[0], bytes[1]]))
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}
impl EndianNumber for u16 {
    fn from_bytes_e(bytes: &[u8], bigendian: bool) -> Option<Self> {
        if bytes.len() != 2 {
            return None;
        }

        let slice = [bytes[0], bytes[1]];

        if bigendian {
            return Some(Self::from_be_bytes(slice));
        }

        Some(Self::from_le_bytes(slice))
    }

    fn to_bytes_e(&self, bigendian: bool) -> Vec<u8> {
        if bigendian {
            return self.to_be_bytes().to_vec();
        }

        return self.to_le_bytes().to_vec();
    }
}

impl SizedNumber for i16 {
    fn size() -> usize {
        2
    }

    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() != 2 {
            return None;
        }

        Some(Self::from_le_bytes([bytes[0], bytes[1]]))
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}
impl EndianNumber for i16 {
    fn from_bytes_e(bytes: &[u8], bigendian: bool) -> Option<Self> {
        if bytes.len() != 2 {
            return None;
        }

        let slice = [bytes[0], bytes[1]];

        if bigendian {
            return Some(Self::from_be_bytes(slice));
        }

        Some(Self::from_le_bytes(slice))
    }

    fn to_bytes_e(&self, bigendian: bool) -> Vec<u8> {
        if bigendian {
            return self.to_be_bytes().to_vec();
        }

        return self.to_le_bytes().to_vec();
    }
}

impl SizedNumber for u32 {
    fn size() -> usize {
        4
    }

    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() != 4 {
            return None;
        }

        Some(Self::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3],
        ]))
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}
impl EndianNumber for u32 {
    fn from_bytes_e(bytes: &[u8], bigendian: bool) -> Option<Self> {
        if bytes.len() != 4 {
            return None;
        }

        let slice = [bytes[0], bytes[1], bytes[2], bytes[3]];

        if bigendian {
            return Some(Self::from_be_bytes(slice));
        }

        Some(Self::from_le_bytes(slice))
    }

    fn to_bytes_e(&self, bigendian: bool) -> Vec<u8> {
        if bigendian {
            return self.to_be_bytes().to_vec();
        }

        return self.to_le_bytes().to_vec();
    }
}

impl SizedNumber for i32 {
    fn size() -> usize {
        4
    }

    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() != 4 {
            return None;
        }

        Some(Self::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3],
        ]))
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}
impl EndianNumber for i32 {
    fn from_bytes_e(bytes: &[u8], bigendian: bool) -> Option<Self> {
        if bytes.len() != 4 {
            return None;
        }

        let slice = [bytes[0], bytes[1], bytes[2], bytes[3]];

        if bigendian {
            return Some(Self::from_be_bytes(slice));
        }

        Some(Self::from_le_bytes(slice))
    }

    fn to_bytes_e(&self, bigendian: bool) -> Vec<u8> {
        if bigendian {
            return self.to_be_bytes().to_vec();
        }

        return self.to_le_bytes().to_vec();
    }
}

impl SizedNumber for f32 {
    fn size() -> usize {
        4
    }

    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() != 4 {
            return None;
        }

        Some(Self::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3],
        ]))
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}
impl EndianNumber for f32 {
    fn from_bytes_e(bytes: &[u8], bigendian: bool) -> Option<Self> {
        if bytes.len() != 4 {
            return None;
        }

        let slice = [bytes[0], bytes[1], bytes[2], bytes[3]];

        if bigendian {
            return Some(Self::from_be_bytes(slice));
        }

        Some(Self::from_le_bytes(slice))
    }

    fn to_bytes_e(&self, bigendian: bool) -> Vec<u8> {
        if bigendian {
            return self.to_be_bytes().to_vec();
        }

        return self.to_le_bytes().to_vec();
    }
}

impl SizedNumber for u64 {
    fn size() -> usize {
        8
    }

    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() != 8 {
            return None;
        }

        Some(Self::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ]))
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}
impl EndianNumber for u64 {
    fn from_bytes_e(bytes: &[u8], bigendian: bool) -> Option<Self> {
        if bytes.len() != 8 {
            return None;
        }

        let slice = [
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ];

        if bigendian {
            return Some(Self::from_be_bytes(slice));
        }

        Some(Self::from_le_bytes(slice))
    }

    fn to_bytes_e(&self, bigendian: bool) -> Vec<u8> {
        if bigendian {
            return self.to_be_bytes().to_vec();
        }

        return self.to_le_bytes().to_vec();
    }
}

impl SizedNumber for i64 {
    fn size() -> usize {
        8
    }

    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() != 8 {
            return None;
        }

        Some(Self::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ]))
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}
impl EndianNumber for i64 {
    fn from_bytes_e(bytes: &[u8], bigendian: bool) -> Option<Self> {
        if bytes.len() != 8 {
            return None;
        }

        let slice = [
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ];

        if bigendian {
            return Some(Self::from_be_bytes(slice));
        }

        Some(Self::from_le_bytes(slice))
    }

    fn to_bytes_e(&self, bigendian: bool) -> Vec<u8> {
        if bigendian {
            return self.to_be_bytes().to_vec();
        }

        return self.to_le_bytes().to_vec();
    }
}

impl SizedNumber for f64 {
    fn size() -> usize {
        8
    }

    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() != 8 {
            return None;
        }

        Some(Self::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ]))
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}
impl EndianNumber for f64 {
    fn from_bytes_e(bytes: &[u8], bigendian: bool) -> Option<Self> {
        if bytes.len() != 8 {
            return None;
        }

        let slice = [
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ];

        if bigendian {
            return Some(Self::from_be_bytes(slice));
        }

        Some(Self::from_le_bytes(slice))
    }

    fn to_bytes_e(&self, bigendian: bool) -> Vec<u8> {
        if bigendian {
            return self.to_be_bytes().to_vec();
        }

        return self.to_le_bytes().to_vec();
    }
}

impl SizedNumber for u128 {
    fn size() -> usize {
        16
    }

    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() != 16 {
            return None;
        }

        Some(Self::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            bytes[8], bytes[9], bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15],
        ]))
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}
impl EndianNumber for u128 {
    fn from_bytes_e(bytes: &[u8], bigendian: bool) -> Option<Self> {
        if bytes.len() != 16 {
            return None;
        }

        let slice = [
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            bytes[8], bytes[9], bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15],
        ];

        if bigendian {
            return Some(Self::from_be_bytes(slice));
        }

        Some(Self::from_le_bytes(slice))
    }

    fn to_bytes_e(&self, bigendian: bool) -> Vec<u8> {
        if bigendian {
            return self.to_be_bytes().to_vec();
        }

        return self.to_le_bytes().to_vec();
    }
}

impl SizedNumber for i128 {
    fn size() -> usize {
        16
    }

    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() != 16 {
            return None;
        }

        Some(Self::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            bytes[8], bytes[9], bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15],
        ]))
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}
impl EndianNumber for i128 {
    fn from_bytes_e(bytes: &[u8], bigendian: bool) -> Option<Self> {
        if bytes.len() != 16 {
            return None;
        }

        let slice = [
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            bytes[8], bytes[9], bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15],
        ];

        if bigendian {
            return Some(Self::from_be_bytes(slice));
        }

        Some(Self::from_le_bytes(slice))
    }

    fn to_bytes_e(&self, bigendian: bool) -> Vec<u8> {
        if bigendian {
            return self.to_be_bytes().to_vec();
        }

        return self.to_le_bytes().to_vec();
    }
}
