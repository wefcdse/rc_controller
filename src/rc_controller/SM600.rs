use hidapi::{HidDevice, HidError};

pub type Sm6 = super::basic_controller::BasicController<HidDevice, HidError, 8>;

#[cfg(test)]
#[test]
fn s() {
    use crate::Controller;

    assert_eq!(Sm6::channels(), 8);
}
////////////////////////
#[allow(unused)]
#[cfg(unused)]
mod unused {
    use std::char::TryFromCharError;

    use hidapi::{HidDevice, HidError};

    use super::super::{error::ControllerError, traits::Controller};

    struct SM600Controller {
        device: hidapi::HidDevice,
        max: [u8; 8],
        min: [u8; 8],
        last_updated: [u8; 8],
    }

    impl Controller for SM600Controller {
        fn update(&mut self) -> super::super::types::ControllerResult<()> {
            let r = self.device.read(&mut self.last_updated[..])?;
            Ok(())
        }

        fn get_output_raw(&self, channel: usize) -> super::super::types::ControllerResult<u8> {
            if Self::has_channel(channel) {
                Ok(self.last_updated[channel])
            } else {
                Err(ControllerError::NoSuchChannel(channel))
            }
        }

        fn get_output(&self, channel: usize) -> super::super::types::ControllerResult<u8> {
            let out = self.get_output_f32(channel)?;
            let out = 255_f32 * out;
            let out = out.round() as u8;
            Ok(out)
        }

        fn get_output_f32(&self, channel: usize) -> super::super::types::ControllerResult<f32> {
            if Self::has_channel(channel) {
                let o = self.last_updated[channel];
                let diff = self.max[channel] - self.min[channel];
                let o_0_1: f32 = (o as f32 - self.min[channel] as f32) / diff as f32;
                Ok(o_0_1)
            } else {
                Err(ControllerError::NoSuchChannel(channel))
            }
        }

        fn set_channel_fix(
            &mut self,
            channel: usize,
            max: u8,
            min: u8,
        ) -> super::super::types::ControllerResult<()> {
            if !Self::has_channel(channel) {
                Err(ControllerError::NoSuchChannel(channel))
            } else {
                self.min[channel] = min;
                self.max[channel] = max;
                Ok(())
            }
        }

        fn channels() -> usize {
            Self::CHANNELS
        }

        fn has_channel(channel: usize) -> bool {
            channel < Self::CHANNELS
        }
    }

    impl SM600Controller {
        const CHANNELS: usize = 8;
        pub fn new(device: HidDevice) -> SM600Controller {
            SM600Controller {
                device,
                max: [u8::MAX; Self::CHANNELS],
                min: [u8::MIN; Self::CHANNELS],
                last_updated: [u8::MIN; Self::CHANNELS],
            }
        }
    }
}
