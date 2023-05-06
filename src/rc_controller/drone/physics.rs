use super::*;

impl Quadrotor {
    // 计算飞机的空气阻力
    pub fn caculate_air_resistance(&self) -> Float {
        // F = (1/2) C ρ S V^2
        let c = self.air_resistance_coefficient;
        let p = self.air_density;
        let v = self.velocity.length();
        // s是飞机的有效的迎风面积
        let s: Float = {
            // 计算速度方向
            let v_unit_vector = match self.velocity.try_normalize() {
                Some(v) => v,
                None => return 0.0,
            };
            let (s_x, s_y, s_z) = {
                let x_unit = Vec3::X;
                let y_unit = Vec3::Y;
                let z_unit = Vec3::Z;

                // 得出旋转后的各面法线方向
                let q = self.orientation;
                let x_unit = q.mul_vec3(x_unit);
                let y_unit = q.mul_vec3(y_unit);
                let z_unit = q.mul_vec3(z_unit);
                dbg!(v_unit_vector);
                dbg!(x_unit, y_unit, z_unit);
                let x = v_unit_vector.cross(x_unit).cross(v_unit_vector);
                let y = v_unit_vector.cross(y_unit).cross(v_unit_vector);
                let z = v_unit_vector.cross(z_unit).cross(v_unit_vector);

                dbg!(x.length(), y.length(), z.length());

                (
                    self.frontal_area_xyz.0 * y.length() * z.length(),
                    self.frontal_area_xyz.1 * x.length() * z.length(),
                    self.frontal_area_xyz.2 * x.length() * y.length(),
                )
            };
            dbg!(s_x, s_y, s_z);
            s_x + s_y + s_z
        };
        0.5 * c * p * s * (v * v)
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
    fn u() {
        let mut q = Quadrotor::new(1.23, 2.13, 1.0, (1.0, 1.0, 1.0));
        q.velocity = Vec3::new(1.0, 1.0, 0.0);
        q.air_density = 2.0;
        println!("{}", q.caculate_air_resistance());
    }
}
