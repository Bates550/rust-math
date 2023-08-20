pub mod line3;
pub mod vec3;

#[cfg(test)]
mod tests {
    use super::line3::Line3;
    use super::vec3::Vec3;

    #[test]
    fn instantiate_vec3() {
        let v1 = Vec3 {
            x: 0.0,
            y: 1.0,
            z: 2.0,
        };
        let v2 = Vec3 {
            x: 3.0,
            y: 4.0,
            z: 5.0,
        };
        let result;

        result = v1 + v2;
        assert_eq!(
            Vec3 {
                x: 3.0,
                y: 5.0,
                z: 7.0
            },
            result
        );
    }

    #[test]
    fn instantiate_line() {
        let _l = Line3 {
            direction: Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            origin: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        };
    }
}
