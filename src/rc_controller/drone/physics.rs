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
        let v = self.velocity.dot(up_unit); // 向上方向的速度为正
        let throttle = self.last_input.0;
        assert!((0.0..=1.0).contains(&throttle));

        let v_engine_out = throttle * self.motor_max_speed;
        let v_real = (v_engine_out - v).max(0.0);
        dbg!(v_engine_out);
        dbg!(v_real);

        let f = self.motor_max_force * v_real / self.motor_max_speed;
        dbg!(f);
        let f = f.min(self.motor_max_force);
        dbg!(f);

        up_unit * f
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
        q.air_density = 2.0;
        q.last_input = (1.0, 0.0, 0.0, 0.0);
        println!("{}", q.caculate_air_resistance());
    }

    #[test]
    fn caculate_engine_force() {
        let mut q = Quadrotor::default();
        q.velocity = Vec3::new(2.0, -259.0, 0.0);
        q.last_input = (0.0, 0.0, 0.0, 0.0);
        println!("{}", q.caculate_engine_force());
    }
}
