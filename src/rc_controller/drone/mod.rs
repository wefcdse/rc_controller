mod types;
pub use types::*;

mod physics;
/// 四翼飞行器
pub struct Quadrotor {
    // front = x
    /// caculated\
    /// m/s
    pub velocity: Vec3,
    /// caculated\
    /// quaternion
    pub orientation: Quat,
    /// caculated\
    /// m/s^2
    pub acceleration: Vec3,
    /// caculated\
    /// N
    pub force_except_gravity: Vec3,

    /// kg
    pub mass: Float,
    /// m/s
    pub motor_max_speed: Float,
    /// N
    pub motor_max_force: Float,

    pub air_resistance_coefficient: Float, //
    /// kg/m^3
    pub air_density: Float,
    /// m^3
    pub frontal_area_xyz: (Float, Float, Float),
    /// throttle(0.0 ~ 1.0), yaw(-1.0 ~ 1.0), pitch(-1.0 ~ 1.0), roll(-1.0 ~ 1.0)
    last_input: (Float, Float, Float, Float),
}

impl Quadrotor {
    pub fn new(
        mass: Float,
        motor_max_speed: Float,
        motor_max_force: Float,
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
            motor_max_force,
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

impl Default for Quadrotor {
    fn default() -> Self {
        Self {
            velocity: Vec3::ZERO,
            orientation: Quat::from_axis_angle(Vec3::X, 0.0),
            acceleration: Vec3::ZERO,
            force_except_gravity: Vec3::ZERO,
            //
            mass: 1.0,
            motor_max_speed: 25.0,
            motor_max_force: 10.0,
            air_resistance_coefficient: 1.0,
            air_density: 1.29,
            frontal_area_xyz: (0.2 * 0.05, 0.04, 0.2 * 0.05),
            //
            last_input: (0.0, 0.0, 0.0, 0.0),
        }
    }
}
