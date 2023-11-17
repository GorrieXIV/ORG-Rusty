pub mod vector;

#[cfg(test)]
mod tests {
    use crate::vector::*;

    #[test]
    fn it_works() {
        let test_vector = Vector2D {
            x: 1.61,
            y: 3.14,
        };

        assert_eq!(test_vector.to_string(), "(1.61, 3.14)");
    }
}
