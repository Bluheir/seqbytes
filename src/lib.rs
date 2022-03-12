pub mod bytes;
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
