mod node;
pub mod algorithm;

pub use node::{Node, Graph};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
