use crate::vec3::Vec3;

#[derive(Debug)]
pub struct Plane {
    normal: Vec3,
    offset: f32,
}

impl Plane {
    pub fn new(normal: Vec3, offset: f32) -> Plane {
        Plane {
            normal: normal.normalize(),
            offset: offset,
        }
    }

    pub fn distance_from(&self, &point: &Vec3) -> f32 {
        // 0 = ax + by + cz - (ax0 + by0 + cz0)
        let a = self.normal.x;
        let b = self.normal.y;
        let c = self.normal.z;
        let p0 = self.normal * self.offset;
        // d = -(ax0 + by0 = cz0)
        let d = -(a * p0.x + b * p0.y + c * p0.z);
        return a * point.x + b * point.y + c * point.z + d;
    }
}

#[cfg(test)]
mod tests {
    use super::Plane;
    use crate::vec3::Vec3;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn distance_from() {
        let p = Plane::new(
            Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            1.0,
        );
        let point_on_plane = Vec3 {
            x: 2.0,
            y: 1.0,
            z: 0.0,
        };
        assert_approx_eq!(p.distance_from(&point_on_plane), 0.0);

        let point_off_plane = Vec3 {
            x: 3.0,
            y: 2.0,
            z: 1.0,
        };
        assert_approx_eq!(p.distance_from(&point_off_plane), 1.0);
    }
}
