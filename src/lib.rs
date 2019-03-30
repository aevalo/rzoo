pub mod librzoo;

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// externing crate for test-only use
#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
}
