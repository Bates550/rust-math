use crate::vec3::Vec3;

pub struct Line3 {
    pub direction: Vec3,
    pub origin: Vec3,
}

impl Line3 {
    pub fn is_point_on_line(&self, &point: &Vec3) -> bool {
        let value_x: f32 = (point.x - self.origin.x) / self.direction.x;
        let value_y: f32 = (point.y - self.origin.y) / self.direction.y;
        let value_z: f32 = (point.z - self.origin.z) / self.direction.z;
        let epsilon: f32 = 0.0001;
        let x_equals_y = f32::abs(value_x - value_y) <= epsilon;
        let x_equals_z = f32::abs(value_x - value_z) <= epsilon;
        return x_equals_y && x_equals_z;
    }
}

#[cfg(test)]
mod tests {
    use crate::vec3::Vec3;

    use super::Line3;

    #[test]
    fn is_point_on_line() {
        let l = Line3 {
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

        let point_on_line = Vec3 {
            x: 2.0,
            y: 2.0,
            z: 2.0,
        };
        assert_eq!(l.is_point_on_line(&point_on_line), true);

        let point_not_on_line = Vec3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };
        assert_eq!(l.is_point_on_line(&point_not_on_line), false);
    }
}
