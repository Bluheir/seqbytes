use std::io::{Read, Seek, SeekFrom};

use super::traits::*;

/// Represents a sequential byte reader which can read bytes. Can be used on types that implement [`Read`] + [`Seek`].
///
/// # Examples
///
/// ```
/// use seqbytes::prelude::*;
/// use std::io::Cursor;
/// use std::str::FromStr;
///
/// let a = vec![69, 96, 255, 255, 0x68, 0x65, 0x6C, 0x6C, 0x6F];
/// let mut cursor = Cursor::new(a);
///
/// let num : i32 = cursor.shift().unwrap();
/// let s = &*cursor.shift_string(5).unwrap();
///
/// assert_eq!(num, -40891);
/// assert_eq!(*s, *"hello");
/// ```
pub trait SeqByteReader {
    /// Peaks the next `U` from the current position, reading the size of `U`'s amount of bytes, and converting to the `U`. Returns [`None`]
    /// if there are not enough bytes to be read.
    fn next<U: SizedNumber>(&mut self) -> Option<U>;
    /// Peaks the next `U` from the current position, shifting and reading the size of `U`'s amount of bytes, and converting to the `U`. Returns [`None`]
    /// if there are not enough bytes to be read.
    ///
    fn shift<U: SizedNumber>(&mut self) -> Option<U>;
    /// Peaks the next `amount` of bytes. Returns a [`Vec<u8>`] containing the bytes.
    fn next_slice(&mut self, amount: usize) -> Option<Vec<u8>>;
    /// Peaks the next `amount` bytes, and shifting the position by `amount` bytes. Returns a [`Vec<u8>`] containing the bytes.
    fn shift_slice(&mut self, amount: usize) -> Option<Vec<u8>>;
    /// Peaks the next `amount` bytes. Returns a [`String`] containing the bytes. Returns [`None`] if there are no
    /// more bytes to be read. If unimplemented, internally calls `next_slice` and converts it to a lossy UTF-8 String.
    fn next_string(&mut self, amount: usize) -> Option<String> {
        let slice = self.next_slice(amount)?;

        Some(String::from_utf8_lossy(&slice).to_string())
    }
    /// Peaks the next `amount` bytes, and shifting the position by `amount` bytes. Returns a [`String`] containing the bytes. Returns [`None`] if there are no
    /// more bytes to be read. If unimplemented, internally calls `shift_slice` and converts it to a lossy UTF-8 String.
    fn shift_string(&mut self, amount: usize) -> Option<String> {
        let slice = self.shift_slice(amount)?;

        Some(String::from_utf8_lossy(&slice).to_string())
    }

    /* Not sure if I should keep these methods. Should I ?
    fn next_u8(&mut self) -> Option<u8> {
        self.next::<u8>()
    }
    fn next_i8(&mut self) -> Option<i8> {
        self.next::<i8>()
    }
    fn next_u16(&mut self) -> Option<u16> {
        self.next::<u16>()
    }
    fn next_i16(&mut self) -> Option<i16> {
        self.next::<i16>()
    }
    fn next_u32(&mut self) -> Option<u32> {
        self.next::<u32>()
    }
    fn next_i32(&mut self) -> Option<i32> {
        self.next::<i32>()
    }
    fn next_f32(&mut self) -> Option<f32> {
        self.next::<f32>()
    }
    fn next_u64(&mut self) -> Option<u64> {
        self.next::<u64>()
    }
    fn next_i64(&mut self) -> Option<i64> {
        self.next::<i64>()
    }
    fn next_f64(&mut self) -> Option<f64> {
        self.next::<f64>()
    }

    fn shift_u8(&mut self) -> Option<u8> {
        self.shift::<u8>()
    }
    fn shift_i8(&mut self) -> Option<i8> {
        self.shift::<i8>()
    }
    fn shift_u16(&mut self) -> Option<u16> {
        self.shift::<u16>()
    }
    fn shift_i16(&mut self) -> Option<i16> {
        self.shift::<i16>()
    }
    fn shift_u32(&mut self) -> Option<u32> {
        self.shift::<u32>()
    }
    fn shift_i32(&mut self) -> Option<i32> {
        self.shift::<i32>()
    }
    fn shift_f32(&mut self) -> Option<f32> {
        self.shift::<f32>()
    }
    fn shift_u64(&mut self) -> Option<u64> {
        self.shift::<u64>()
    }
    fn shift_i64(&mut self) -> Option<i64> {
        self.shift::<i64>()
    }
    fn shift_f64(&mut self) -> Option<f64> {
        self.shift::<f64>()
    }
    */
}
/// Represents a sequential byte reader which can read bytes with a specified endianness. Can be used on types that implement [`Read`] + [`Seek`]
///
/// # Examples
///
/// ```
/// use seqbytes::prelude::*;
/// use std::io::Cursor;
///
/// let a = vec![69, 96, 255, 255];
/// let mut cursor = Cursor::new(a);
///
/// let num : i32 = cursor.next_e(false).unwrap();
/// let num2 : i32 = cursor.shift_e(true).unwrap();
/// let num3 : Option<i32> = cursor.shift_e(false);
///
/// assert_ne!(num, num2);
/// assert_eq!(num, -40891);
/// assert_eq!(num2, 1163984895);
/// assert_eq!(num3, None);
/// ```
pub trait ESeqByteReader {
    /// Peaks the next `U` from the current position, reading the size of `U`'s amount of bytes, and converting to the `U` with the specified endianness. Returns [`None`]
    /// if there are not enough bytes to be read.
    ///
    /// # Examples
    ///
    /// ```
    /// use seqbytes::prelude::*;
    /// use std::io::Cursor;
    ///
    /// let a = vec![69, 96, 255, 255];
    /// let mut cursor = Cursor::new(a);
    ///
    /// let pos1 = cursor.position();
    /// let num : i32 = cursor.next_e(false).unwrap();
    /// let pos2 = cursor.position();
    ///
    /// assert_eq!(pos1, pos2);
    /// assert_eq!(num, -40891);
    /// ```
    fn next_e<U: EndianNumber>(&mut self, bigendian: bool) -> Option<U>;
    /// Peaks the next `U` from the current position, shifting and reading the size of `U`'s amount of bytes, and converting to the `U` with the specified endianness. Returns [`None`]
    /// if there are not enough bytes to be read.
    ///
    /// # Examples
    ///
    /// ```
    /// use seqbytes::prelude::*;
    /// use std::io::Cursor;
    ///
    /// let a = vec![69, 96, 255, 255];
    /// let mut cursor = Cursor::new(a);
    ///
    /// let pos1 = cursor.position();
    /// let num : i32 = cursor.shift_e(false).unwrap();
    /// let pos2 = cursor.position();
    ///
    /// assert_ne!(pos1, pos2);
    /// assert_eq!(num, -40891);
    /// ```
    fn shift_e<U: EndianNumber>(&mut self, bigendian: bool) -> Option<U>;
}

impl<T: Seek + Read> SeqByteReader for T {
    fn next<U: SizedNumber>(&mut self) -> Option<U> {
        let size = U::size() as isize;

        let mut a = vec![0u8; size as usize];
        self.read_exact(&mut a).ok()?;

        self.seek(SeekFrom::Current(-size as i64)).unwrap(); // Should not panic, as it is shifting backwards the same amount of bytes as moving forward.

        return U::from_bytes(&a[..]);
    }

    fn shift<U: SizedNumber>(&mut self) -> Option<U> {
        let size = U::size() as isize;

        let mut a = vec![0u8; size as usize];
        self.read_exact(&mut a).ok()?;

        return U::from_bytes(&a[..]);
    }

    fn next_slice(&mut self, amount: usize) -> Option<Vec<u8>> {
        let mut a = vec![0u8; amount];
        self.read_exact(&mut a).ok()?;

        self.seek(SeekFrom::Current(-(amount as i64))).unwrap();

        return Some(a);
    }

    fn shift_slice(&mut self, amount: usize) -> Option<Vec<u8>> {
        let mut a = vec![0u8; amount];
        self.read_exact(&mut a).ok()?;

        return Some(a);
    }
}
impl<T: Seek + Read> ESeqByteReader for T {
    fn next_e<U: EndianNumber>(&mut self, bigendian: bool) -> Option<U> {
        let size = U::size() as isize;

        let mut a = vec![0u8; size as usize];
        self.read_exact(&mut a).ok()?;

        self.seek(SeekFrom::Current(-size as i64)).unwrap(); // Should not panic, as it is shifting backwards the same amount of bytes as moving forward.

        return U::from_bytes_e(&a[..], bigendian);
    }

    fn shift_e<U: EndianNumber>(&mut self, bigendian: bool) -> Option<U> {
        let size = U::size() as isize;

        let mut a = vec![0u8; size as usize];
        self.read_exact(&mut a).ok()?;

        return U::from_bytes_e(&a[..], bigendian);
    }
}
