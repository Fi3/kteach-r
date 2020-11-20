#![feature(associated_type_bounds)]
pub mod decoder;
pub mod engine;
pub mod midi;
pub mod modules;
pub mod output;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
