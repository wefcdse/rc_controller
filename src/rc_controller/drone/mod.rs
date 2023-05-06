mod types;
pub use types::*;

mod physics;

pub struct Quadrotor {
    // front = x
    // caculated
    pub velocity: Vec3,     // m/s
    pub orientation: Quat,  // quaternion
    pub acceleration: Vec3, // m/s^2
    pub force_except_gravity: Vec3,
    //
    pub mass: Float,                             // kg
    pub motor_max_speed: Float,                  // m/s
    pub air_resistance_coefficient: Float,       //
    pub air_density: Float,                      // kg/m^3
    pub frontal_area_xyz: (Float, Float, Float), // m^3
    //
    last_input: (Float, Float, Float, Float), // throttle, yaw, pitch, roll
}

impl Quadrotor {
    pub fn new(
        mass: Float,
        motor_max_speed: Float,
        air_resistance_coefficient: Float,
        frontal_area_xyz: (Float, Float, Float),
    ) -> Self {
        Self {
            velocity: Vec3::ZERO,
            orientation: Quat::from_axis_angle(Vec3::X, 0.0),
            acceleration: Vec3::ZERO,
            force_except_gravity: Vec3::ZERO,
            //
            mass,
            motor_max_speed,
            air_resistance_coefficient,
            air_density: 1.29,
            frontal_area_xyz,
            //
            last_input: (0.0, 0.0, 0.0, 0.0),
        }
    }

    pub fn update_input(
        &mut self,
        throttle: f32,
        yaw: f32,
        pitch: f32,
        roll: f32,
    ) -> (Float, Float, Float, Float) {
        let throttle = throttle as Float;
        let yaw = yaw as Float;
        let pitch = pitch as Float;
        let roll = roll as Float;
        self.last_input = (throttle, yaw, pitch, roll);
        self.last_input
    }
}
