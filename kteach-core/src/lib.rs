#![feature(associated_type_bounds)]
pub mod decoder;
pub mod midi;
pub mod output;
pub mod player;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
