use std::f32::consts::FRAC_1_SQRT_2;

use bevy::math::Vec3;

pub const FIRST_ORDER_POS: [Vec3; 9] = [
    Vec3::new(0., 1., 0.), // TOP
    Vec3::new(1., 0., 0.),
    Vec3::new(-1., 0., 0.),
    Vec3::new(0., 0., 1.),  // FRONT
    Vec3::new(0., 0., -1.), // BEHIND
    Vec3::new(-0.5, FRAC_1_SQRT_2, -0.5),
    Vec3::new(-0.5, FRAC_1_SQRT_2, 0.5),
    Vec3::new(0.5, FRAC_1_SQRT_2, -0.5),
    Vec3::new(0.5, FRAC_1_SQRT_2, 0.5),
];
pub const SECOND_ORDER_POS: [Vec3; 25] = [
    // FIRST ORDER
    Vec3::new(0., 1., 0.), // TOP
    Vec3::new(1., 0., 0.),
    Vec3::new(-1., 0., 0.),
    Vec3::new(0., 0., 1.),  // FRONT
    Vec3::new(0., 0., -1.), // BEHIND
    Vec3::new(-0.5, FRAC_1_SQRT_2, -0.5),
    Vec3::new(-0.5, FRAC_1_SQRT_2, 0.5),
    Vec3::new(0.5, FRAC_1_SQRT_2, -0.5),
    Vec3::new(0.5, FRAC_1_SQRT_2, 0.5),
    // ADDITIONAL SECOND ORDER
    Vec3::new(0., FRAC_1_SQRT_2, FRAC_1_SQRT_2),
    Vec3::new(0., FRAC_1_SQRT_2, -FRAC_1_SQRT_2),
    Vec3::new(FRAC_1_SQRT_2, FRAC_1_SQRT_2, 0.),
    Vec3::new(-FRAC_1_SQRT_2, FRAC_1_SQRT_2, 0.),
    Vec3::new(FRAC_1_SQRT_2, 0., FRAC_1_SQRT_2),
    Vec3::new(-FRAC_1_SQRT_2, 0., FRAC_1_SQRT_2),
    Vec3::new(-FRAC_1_SQRT_2, 0., -FRAC_1_SQRT_2),
    Vec3::new(FRAC_1_SQRT_2, 0., -FRAC_1_SQRT_2),
    Vec3::new(0., -FRAC_1_SQRT_2, FRAC_1_SQRT_2),
    Vec3::new(0., -FRAC_1_SQRT_2, -FRAC_1_SQRT_2),
    Vec3::new(FRAC_1_SQRT_2, -FRAC_1_SQRT_2, 0.),
    Vec3::new(-FRAC_1_SQRT_2, -FRAC_1_SQRT_2, 0.),
    Vec3::new(-0.5, -FRAC_1_SQRT_2, -0.5),
    Vec3::new(-0.5, -FRAC_1_SQRT_2, 0.5),
    Vec3::new(0.5, -FRAC_1_SQRT_2, -0.5),
    Vec3::new(0.5, -FRAC_1_SQRT_2, 0.5),
];
