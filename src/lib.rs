//! # The seqbytes library
//! The seqbytes crate provides the traits `ESeqByteReader` and `SeqByteReader` used for reading bytes sequentially. The `SeqByteReader` trait convert the bytes into the
//! specified generic type, denoted `U`, which must implement `SizedNumber`. The trait `SizedNumber` represents a type which can be converted to and from bytes, with a
//! fixed size in bytes.
//! 
//! The trait `ESeqByteReader` is used for reading bytes sequentially, converting to a type with a specific endianness. The type converted to must implement `EndianNumber`,
//! which represents a type which can be converted to and from bytes with a specific endianness.
//! 
//! # Implementation
//! The traits `E$eqByteReader` and `SeqByteReader` are implemented by default on types implementing `Read` + `Seek`.
//! 
//! ## Example 1
//! Using [`SizedNumber`] trait to convert numbers.
//! ```
//! use seqbytes::traits::*;
//!
//! let a = 22.4f64;
//! let b = a.to_bytes();
//! let c = f64::from_bytes(&b);
//! let d = u32::from_bytes(&[2, 4, 6]);
//! assert_eq!(c, Some(22.4));
//! assert_eq!(d, None);
//! ```
//! ## Example 2
//! Using [`crate::bytes::SeqByteReader`] to read a Vector sequentially
//! ```
//! use seqbytes::prelude::*;
//! use std::io::Cursor;
//! use std::str::FromStr;
//!
//! let a = vec![69, 96, 255, 255, 0x68, 0x65, 0x6C, 0x6C, 0x6F];
//! let mut cursor = Cursor::new(a);
//!
//! let num : i32 = cursor.shift().unwrap();
//! let s = &*cursor.shift_string(5).unwrap();
//!
//! assert_eq!(num, -40891);
//! assert_eq!(*s, *"hello");
//! ```

/// Contains the traits [`crate::bytes::SeqByteReader`] and [`crate::bytes::ESeqByteReader`]
pub mod bytes;
/// Re-exports everything from the module [`crate::bytes`] and [`crate::traits`]
pub mod prelude;
/// Contains all traits in this library.
pub mod traits;

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn f64_trait_test() {
        use crate::traits::*;

        let a = 22.4f64;
        let b = a.to_bytes();
        let c = f64::from_bytes_e(&b, false).unwrap();

        assert_eq!(c, 22.4);
    }
    #[test]
    fn tt() {
        let a = [69, 96, 255, 255];
        let b = i32::from_le_bytes(a);
        let c = i32::from_be_bytes(a);

        println!("{}", b);
        println!("{}", c);
    }

    #[test]
    fn endiantest() {
        use crate::prelude::*;
        use std::io::Cursor;

        let a = vec![69, 96, 255, 255, 0x68, 0x65, 0x6C, 0x6C, 0x6F];
        let mut cursor = Cursor::new(a);

        let num: i32 = cursor.shift().unwrap();
        let s = &*cursor.shift_string(5).unwrap();

        assert_eq!(num, -40891);
        assert_eq!(*s, *"hello");
    }
}