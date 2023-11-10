pub mod entity;

#[cfg(test)]
mod tests {
    use crate::entity::*;

    #[test]
    fn it_works() {
        let test_entity = Entity {
            x: 0,
            y: 0,
            width: 10,
            height: 10,
            angle: 0,
            collision_detected: false,
        };

        test_entity.set_position(7, 14);

        assert_eq!(test_entity.x, 7);
        assert_eq!(test_entity.y, 14);
    }
}
