#![feature(associated_type_bounds)]
pub mod decoder;
pub mod output;
pub mod player;
pub mod source;
pub mod state;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
