use crate::util::{clear, CanSend, CanSync};
use crate::{fpv_controller::BasicFPVController, Controller, ControllerUtils, FixType, SM600};
use hidapi::{DeviceInfo, HidDevice};
use std::sync::Mutex;
use std::time::Instant;
pub fn simple_loader<'a>(time: f32) -> BasicFPVController<'a> {
    {
        let hid_api = hidapi::HidApi::new().unwrap();
        let mut devices = Vec::<&DeviceInfo>::new();
        for (id, d) in hid_api.device_list().enumerate() {
            println!("{id} {:?} {:?}", d, d.product_string());

            devices.push(d);
        }
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        let s: String = s.chars().filter(|s| s.is_digit(10)).collect();
        let n: usize = s.parse().unwrap();
        println!("{n}");
        let device = devices[n].open_device(&hid_api).unwrap();
        device.can_send_i();
        // device.can_sync_i();
        Mutex::<HidDevice>::can_sync();
        let mut c = SM600::Sm6::new(device);
        c.can_send_i();
        // c.can_sync_i();
        Mutex::<SM600::Sm6>::can_send();
        Mutex::<SM600::Sm6>::can_sync();

        for i in 0..c.channels() {
            c.set_channel_fix(i, Some(0), Some(255), Some(127.0))
                .unwrap();
            c.set_fix_type(i, FixType::MaxMidMin).unwrap();
        }
        c.set_fix_type(3, FixType::MaxMin).unwrap();

        let mut _mid = [127.0_f32; 8];
        let now = Instant::now();
        'out: loop {
            c.update_and_fix(0.01).unwrap();
            clear();
            for i in 0..c.channels() {
                if now.elapsed().as_secs_f32() < time {
                    // c.read_and_fix_f32_max_min(i).unwrap();
                    // c.read_and_fix_f32_mid(i, 0.01).unwrap();
                } else {
                    break 'out;
                }

                let mid = c.get_channel_fix_mid(i).unwrap();
                println!(
                    "{i} {:.3} {:.3} raw:{}",
                    c.get_output(i).unwrap(),
                    mid,
                    c.get_output_raw(i).unwrap()
                );
                println!("");
            }
        }

        let mut c1 = BasicFPVController::new(c);
        c1.set_channels(Some(3), Some(5), Some(2), Some(1)).unwrap();
        c1.init().unwrap();
        c1.can_send_i();
        // c1.can_sync_i();
        let now = Instant::now();
        loop {
            c1.update().unwrap();
            clear();
            let t = c1.get_throttle().unwrap();
            let y = c1.get_yaw().unwrap();
            let p = c1.get_pitch().unwrap();
            let r = c1.get_roll().unwrap();
            println!("{:.2}\n {:.2}\n {:.2}\n {:.2}", t, y, p, r);
            if now.elapsed().as_secs_f32() > time {
                return c1;
            }
        }
    }
}
