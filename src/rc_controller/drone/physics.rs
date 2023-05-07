use std::time::Duration;

use super::*;

impl Quadrotor {
    // 计算飞机的空气阻力
    pub fn caculate_air_resistance(&self) -> Vec3 {
        // F = (1/2) C ρ S V^2
        let c = self.air_resistance_coefficient;
        let p = self.air_density;
        let v = self.velocity.length();

        // 计算速度方向
        let v_unit_vector = match self.velocity.try_normalize() {
            Some(v) => v,
            None => return Vec3::ZERO,
        };

        // s是飞机的有效的迎风面积
        let s: Float = {
            let (s_x, s_y, s_z) = {
                let x_unit = Vec3::X;
                let y_unit = Vec3::Y;
                let z_unit = Vec3::Z;

                // 得出旋转后的各面法线方向
                let q = self.orientation;
                let x_unit = q.mul_vec3(x_unit);
                let y_unit = q.mul_vec3(y_unit);
                let z_unit = q.mul_vec3(z_unit);
                // dbg!(v_unit_vector);
                // dbg!(x_unit, y_unit, z_unit);
                let x = v_unit_vector.dot(x_unit);
                let y = v_unit_vector.dot(y_unit);
                let z = v_unit_vector.dot(z_unit);

                // dbg!(x, y, z);

                (
                    self.frontal_area_xyz.0 * x,
                    self.frontal_area_xyz.1 * y,
                    self.frontal_area_xyz.2 * z,
                )
            };
            // dbg!(s_x, s_y, s_z);
            s_x + s_y + s_z
        };
        0.5 * c * p * s * (v * v) * -v_unit_vector
    }

    pub fn caculate_engine_force(&self) -> Vec3 {
        let up_unit = self.orientation.mul_vec3(Vec3::Y);
        let throttle = self.last_input.0;
        let throttle = throttle.max(0.0).min(1.0);
        assert!((0.0..=1.0).contains(&throttle));

        up_unit * throttle * self.motor_max_force
    }

    /// m/s^2
    pub fn caculate_total_force_except_g(&self) -> Vec3 {
        self.caculate_air_resistance() + self.caculate_engine_force()
    }

    /// m/s^2
    pub fn caculate_acceleration(&self) -> Vec3 {
        self.caculate_total_force_except_g() / self.mass + self.g
    }

    pub fn update_v(&mut self, dur: Duration) {
        let time_s: Float = dur.as_secs_f64().into();
        self.velocity += self.caculate_acceleration() * time_s;
    }

    pub fn update_orientation(&mut self, dur: Duration) {
        let o = self.orientation;

        let time_s: Float = dur.as_secs_f64().into();

        let yaw_a = self.angular_velocity.0 * self.last_input.1;
        let pitch_a = self.angular_velocity.1 * self.last_input.2;
        let roll_a = self.angular_velocity.2 * self.last_input.3;

        let x = self.orientation.mul_vec3(Vec3::X);
        let y = self.orientation.mul_vec3(Vec3::Y);
        let z = self.orientation.mul_vec3(Vec3::Z);

        let yaw_o = Quat::from_axis_angle(y, yaw_a * time_s);
        let pitch_o = Quat::from_axis_angle(z, pitch_a * time_s);
        let roll_o = Quat::from_axis_angle(x, roll_a * time_s);

        let o = yaw_o.mul_quat(o);
        let o = pitch_o.mul_quat(o);
        let o = roll_o.mul_quat(o);

        self.orientation = o;
    }

    pub fn update_phy(&mut self, dur: Duration) {
        self.update_v(dur);
        self.update_orientation(dur);
    }
}

#[test]
fn t() {
    let a = Vec3::new(1.0, 0.0, 0.0);
    let b = Vec3::new(1.0, 1.0, 0.0);
    let _c = 2.0 * b;
    println!("{}", a.cross(b));
    println!("{}", a.project_onto(b));
    println!("{}", a.reject_from(b));
}
#[cfg(test)]
mod t {
    use super::*;
    #[test]
    fn caculate_air_resistance() {
        let mut q = Quadrotor::new(1.23, 2.13, 1.0, 1.0, (1.0, 1.0, 1.0));
        q.velocity = Vec3::new(1.0, 1.0, 0.0);

        q.orientation =
            Quat::from_axis_angle(Vec3::Z, std::f32::consts::PI as Float * 45.0 / 180.0);
        q.air_density = 2.0;
        q.last_input = (1.0, 0.0, 0.0, 0.0);
        println!("{}", q.caculate_air_resistance());
        println!("{}", q.caculate_air_resistance().length());
    }

    #[test]
    fn caculate_engine_force() {
        let mut q = Quadrotor::default();
        q.velocity = Vec3::new(0.0, 30.0, 0.0);
        q.last_input = (1.0, 0.0, 0.0, 0.0);
        q.orientation =
            Quat::from_axis_angle(Vec3::X, std::f32::consts::PI as Float * 45.0 / 180.0);
        println!("{}", q.caculate_air_resistance());
        println!("{}", q.caculate_air_resistance().length());
        println!("{}", q.caculate_engine_force());
        println!("{}", q.caculate_engine_force().length());
    }

    #[test]
    fn caculate_total_force() {
        let mut q = Quadrotor::default();
        q.velocity = Vec3::new(20.0, 0.0, 0.0);
        q.last_input = (1.0, 0.0, 0.0, 0.0);
        q.orientation =
            Quat::from_axis_angle(Vec3::Z, std::f32::consts::PI as Float * -85.0 / 180.0);
        println!("{}", q.caculate_air_resistance());
        println!("{}", q.caculate_air_resistance().length());
        println!("{}", q.caculate_engine_force());
        println!("{}", q.caculate_engine_force().length());
        println!("{}", q.caculate_total_force_except_g());
        println!("{}", q.caculate_total_force_except_g().length());
    }

    #[test]
    fn caculate_acceleration() {
        let mut q = Quadrotor::default();
        q.velocity = Vec3::new(0.0, 0.0, 0.0);
        q.last_input = (1.0, 0.0, 0.0, 0.0);
        q.orientation =
            Quat::from_axis_angle(Vec3::Z, std::f32::consts::PI as Float * -0.0 / 180.0);
        println!("{}", q.caculate_air_resistance());
        println!("{}", q.caculate_air_resistance().length());
        println!("{}", q.caculate_engine_force());
        println!("{}", q.caculate_engine_force().length());
        println!("{}", q.caculate_total_force_except_g());
        println!("{}", q.caculate_total_force_except_g().length());
        println!("{}", q.caculate_acceleration());
        println!("{}", q.caculate_acceleration().length());
    }
}
