use std::ops;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    /**
     * Vector Length
     */

    pub fn length(&self) -> f32 {
        let x = self.x;
        let y = self.y;
        let z = self.z;
        f32::sqrt(x * x + y * y + z * z)
    }

    /**
     * Vector Length Squared
     */

    pub fn length_squared(&self) -> f32 {
        let x = self.x;
        let y = self.y;
        let z = self.z;
        x * x + y * y + z * z
    }

    /**
     * Vector Normalize
     */

    pub fn normalize(&self) -> Vec3 {
        let length_sq = self.length_squared();
        if length_sq == 0.0 {
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
        }
        let reciprocal = 1.0 / f32::sqrt(length_sq);
        Vec3 {
            x: self.x * reciprocal,
            y: self.y * reciprocal,
            z: self.z * reciprocal,
        }
    }

    /**
     * Vector Absolute (same as length)
     */

    pub fn abs(&self) -> f32 {
        self.length()
    }

    /**
     * Vector Dot
     */

    pub fn dot(&left: &Vec3, &right: &Vec3) -> f32 {
        return left.x * right.x + left.y * right.y + left.z * right.z;
    }

    /**
     * Vector Cross
     */

    pub fn cross(&left: &Vec3, &right: &Vec3) -> Vec3 {
        return Vec3 {
            x: left.y * right.z - right.y * left.z,
            y: left.z * right.x - right.z * left.x,
            z: left.x * right.y - right.x * left.y,
        };
    }

    /**
     * Vector Fast Cross (maybe faster?)
     */

    pub fn fast_cross(&left: &Vec3, &right: &Vec3) -> Vec3 {
        let vec1 = Vec3 {
            x: left.y * right.z,
            y: left.z * right.x,
            z: left.x * right.y,
        };
        let vec2 = Vec3 {
            x: right.y * left.z,
            y: right.z * left.x,
            z: right.x * left.y,
        };
        return vec1 - vec2;
    }

    /**
     * Convert cartesian (x, y, z) to spherical (rho, phi, theta)
     */

    pub fn to_spherical(&self) -> Vec3 {
        let x2 = self.x * self.x;
        let y2 = self.y * self.y;
        Vec3 {
            // rho
            x: f32::sqrt(x2 + y2 + self.z * self.z),
            // phi (zenith)
            y: f32::atan2(f32::sqrt(x2 + y2), self.z),
            // theta (azimuth)
            z: f32::atan2(self.y, self.x),
        }
    }

    /**
     * Convert spherical (rho, phi, theta) to cartesian (x, y, z)
     */

    pub fn to_cartesian(&self) -> Vec3 {
        let rho = self.x;
        let phi = self.y;
        let theta = self.z;
        Vec3 {
            x: rho * f32::sin(phi) * f32::cos(theta),
            y: rho * f32::sin(phi) * f32::sin(theta),
            z: rho * f32::cos(phi),
        }
    }
}

/**
 * Vector Addition
 */

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

/**
 * Vector Subtraction
 */

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

/**
 * Unary Negation
 */

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        -1.0 * self
    }
}

/**
 * Scalar Multiply/Divide
 */

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, scalar: f32) -> Vec3 {
        Vec3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, scalar: Vec3) -> Vec3 {
        Vec3 {
            x: self * scalar.x,
            y: self * scalar.y,
            z: self * scalar.z,
        }
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, scalar: f32) -> Vec3 {
        Vec3 {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

impl ops::Div<Vec3> for f32 {
    type Output = Vec3;

    fn div(self, scalar: Vec3) -> Vec3 {
        Vec3 {
            x: self / scalar.x,
            y: self / scalar.y,
            z: self / scalar.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Vec3;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn vector_addition() {
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

        // Vec3 + Vec3
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
    fn vector_subtraction() {
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

        // Vec3 - Vec3
        result = v1 - v2;
        assert_eq!(
            Vec3 {
                x: -3.0,
                y: -3.0,
                z: -3.0
            },
            result
        );
    }

    #[test]
    fn unary_negation() {
        assert_eq!(
            -(Vec3 {
                x: 0.0,
                y: -1.0,
                z: 2.0
            }),
            Vec3 {
                x: 0.0,
                y: 1.0,
                z: -2.0
            }
        );
    }

    #[test]
    fn scalar_multiplication() {
        let v1 = Vec3 {
            x: -1.0,
            y: 0.0,
            z: 2.0,
        };
        let mut result: Vec3;

        // Vec3 * f32
        result = v1 * 2.0;
        assert_eq!(
            Vec3 {
                x: -2.0,
                y: 0.0,
                z: 4.0,
            },
            result
        );

        // f32 * Vec3
        result = 2.0 * v1;
        assert_eq!(
            Vec3 {
                x: -2.0,
                y: 0.0,
                z: 4.0,
            },
            result
        );

        // Vec3 / f32
        result = v1 / 2.0;
        assert_eq!(
            Vec3 {
                x: -0.5,
                y: 0.0,
                z: 1.0,
            },
            result
        );

        // f32 / Vec3
        result = 2.0 / v1;
        println!("${:?}", result);
        assert_eq!(
            Vec3 {
                x: -2.0,
                y: f32::INFINITY,
                z: 1.0,
            },
            result
        );
    }

    #[test]
    fn vector_length() {
        let v1 = Vec3 {
            x: -f32::sqrt(2.0) / 2.0,
            y: 0.0,
            z: f32::sqrt(2.0) / 2.0,
        };
        let result: f32;

        result = v1.length();
        assert_approx_eq!(1.0, result);
    }

    #[test]
    fn vector_length_squared() {
        let v1 = Vec3 {
            x: 1.0,
            y: 2.0,
            z: -3.0,
        };
        let result: f32;

        result = v1.length_squared();
        assert_approx_eq!(14.0, result);
    }

    #[test]
    fn vector_normalize() {
        let v1 = Vec3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };
        let mut result: Vec3;

        result = v1.normalize();
        assert_eq!(
            Vec3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            result
        );
        assert_approx_eq!(1.0, result.length());

        let v2 = Vec3 {
            x: 1.0,
            y: 2.0,
            z: -3.0,
        };
        result = v2.normalize();

        assert_eq!(
            Vec3 {
                x: 0.26726124,
                y: 0.5345225,
                z: -0.8017837
            },
            result
        );
        assert_approx_eq!(1.0, result.length());
    }

    #[test]
    fn dot_product() {
        let v1 = Vec3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };
        let v2 = Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };
        let mut result: f32;

        // Two orthogonal vectors
        result = Vec3::dot(&v1, &v2);
        assert_eq!(0.0, result);

        // Two equal vectors
        result = Vec3::dot(&v1, &v1);
        assert_eq!(1.0, result);

        // Two opposite vectors
        result = Vec3::dot(&(v1 * -1.0), &v1);
        assert_eq!(-1.0, result);
    }

    #[test]
    fn cross_product() {
        let v1 = Vec3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };
        let v2 = Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };
        let zero_vec = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let mut result: Vec3;

        // Two orthogonal vectors
        result = Vec3::cross(&v1, &v2);
        assert_eq!(
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
            result
        );

        // Two different vectors
        result = Vec3::cross(
            &v1,
            &Vec3 {
                x: 0.0,
                y: 1.0,
                z: 1.0,
            },
        );
        assert_eq!(
            Vec3 {
                x: 0.0,
                y: -1.0,
                z: 1.0,
            },
            result
        );

        // Two equal vectors
        result = Vec3::cross(&v1, &v1);
        assert_eq!(
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            result
        );

        // Two opposite vectors
        result = Vec3::cross(&(v1 * -1.0), &v1);
        assert_eq!(zero_vec, result);

        // v1 x v2 = -(v2 x v1)
        assert_eq!(Vec3::cross(&v1, &v2), -1.0 * Vec3::cross(&v2, &v1));

        // v x 0 = 0
        assert_eq!(Vec3::cross(&v1, &zero_vec), zero_vec);
    }

    #[test]
    fn fast_cross_product() {
        let v1 = Vec3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };
        let v2 = Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };
        let zero_vec = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let mut result: Vec3;

        // Two orthogonal vectors
        result = Vec3::fast_cross(&v1, &v2);
        assert_eq!(
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
            result
        );

        // Two different vectors
        result = Vec3::fast_cross(
            &v1,
            &Vec3 {
                x: 0.0,
                y: 1.0,
                z: 1.0,
            },
        );
        assert_eq!(
            Vec3 {
                x: 0.0,
                y: -1.0,
                z: 1.0,
            },
            result
        );

        // Two equal vectors
        result = Vec3::fast_cross(&v1, &v1);
        assert_eq!(
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            result
        );

        // Two opposite vectors
        result = Vec3::fast_cross(&(v1 * -1.0), &v1);
        assert_eq!(zero_vec, result);

        // v1 x v2 = -(v2 x v1)
        assert_eq!(
            Vec3::fast_cross(&v1, &v2),
            -1.0 * Vec3::fast_cross(&v2, &v1)
        );

        // v x 0 = 0
        assert_eq!(Vec3::fast_cross(&v1, &zero_vec), zero_vec);
    }

    #[test]
    fn to_spherical() {
        // zero vector
        assert_eq!(
            (Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            })
            .to_spherical(),
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }
        );

        assert_approx_eq!(
            (Vec3 {
                x: f32::sqrt(1.0 / 3.0),
                y: f32::sqrt(1.0 / 3.0),
                z: f32::sqrt(1.0 / 3.0),
            })
            .to_spherical(),
            Vec3 {
                x: 0.99999994,
                y: 0.9553166,
                z: 0.7853982,
            }
        );
    }

    #[test]
    fn to_cartesian() {
        // zero vector
        assert_eq!(
            (Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            })
            .to_cartesian(),
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }
        );

        assert_approx_eq!(
            (Vec3 {
                x: 0.99999994,
                y: 0.9553166,
                z: 0.7853982,
            })
            .to_cartesian(),
            Vec3 {
                x: f32::sqrt(1.0 / 3.0),
                y: f32::sqrt(1.0 / 3.0),
                z: f32::sqrt(1.0 / 3.0),
            }
        );
    }
}
