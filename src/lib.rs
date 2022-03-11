pub mod bytes;
pub mod prelude;
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
}
