use std::marker::PhantomData;

use super::{Controller, ControllerError, ControllerUtils, FixType};

pub trait ReadData {
    type Error: Into<ControllerError>;
    fn read_data(&self, buf: &mut [u8]) -> Result<usize, Self::Error>;
}

#[derive(Debug, Clone)]
pub struct BasicController<Device, Err, const CH: usize> {
    p: PhantomData<Err>,
    device: Device,
    max: [u8; CH],
    min: [u8; CH],
    mid: [f32; CH],

    last_updated: [u8; CH],
    fix_type: [FixType; CH],
}

impl<Device, Err, const CH: usize> Controller for BasicController<Device, Err, CH>
where
    Device: ReadData<Error = Err>,
    Err: Into<ControllerError>,
{
    fn update(&mut self) -> super::types::ControllerResult<()> {
        let _r = match self.device.read_data(&mut self.last_updated[..]) {
            Ok(a) => a,
            Err(e) => {
                return Err(e.into());
            }
        };
        Ok(())
    }

    fn get_output_raw(&self, channel: usize) -> super::types::ControllerResult<u8> {
        if self.has_channel(channel) {
            Ok(self.last_updated[channel])
        } else {
            Err(ControllerError::NoSuchChannel(channel))
        }
    }

    fn get_output(&self, channel: usize) -> super::types::ControllerResult<u8> {
        let out = self.get_output_f32(channel)?;
        let out = 255_f32 * out;
        let out = out.round() as u8;
        Ok(out)
    }

    fn get_output_f32(&self, channel: usize) -> super::types::ControllerResult<f32> {
        if self.has_channel(channel) {
            match self.fix_type[channel] {
                FixType::MaxMin => {
                    let o = self.last_updated[channel];
                    let diff = self.max[channel] - self.min[channel];
                    let o_0_1: f32 = (o as f32 - self.min[channel] as f32) / diff as f32;
                    Ok(o_0_1)
                }
                FixType::MaxMidMin => {
                    let o = self.last_updated[channel];
                    let mid = self.mid[channel];
                    let max = self.max[channel];
                    let min = self.min[channel];
                    Ok(match o {
                        v if (v as f32) < mid => {
                            let d = mid - min as f32;
                            ((v - min) as f32) / d * 0.5
                        }
                        v if v as f32 == mid => 0.5,
                        v if (v as f32) > mid => {
                            let d = max as f32 - mid;
                            ((v as f32 - mid) as f32) / d * 0.5 + 0.5
                        }
                        _ => panic!("why here?"),
                    })
                }
                FixType::None => {
                    let o_0_1: f32 = (self.last_updated[channel] as f32) / 255.0;
                    Ok(o_0_1)
                }
            }
        } else {
            Err(ControllerError::NoSuchChannel(channel))
        }
    }

    fn set_channel_fix(
        &mut self,
        channel: usize,
        max: Option<u8>,
        min: Option<u8>,
        mid: Option<f32>,
    ) -> super::types::ControllerResult<()> {
        if !self.has_channel(channel) {
            Err(ControllerError::NoSuchChannel(channel))
        } else {
            match min {
                Some(v) => self.min[channel] = v,
                None => {}
            };
            match max {
                Some(v) => self.max[channel] = v,
                None => {}
            };
            match mid {
                Some(v) => self.mid[channel] = v,
                None => {}
            };
            Ok(())
        }
    }

    fn channels(&self) -> usize {
        Self::CHANNELS
    }

    fn get_channel_fix_max(&mut self, channel: usize) -> crate::ControllerResult<u8> {
        if !self.has_channel(channel) {
            Err(ControllerError::NoSuchChannel(channel))
        } else {
            Ok(self.max[channel])
        }
    }

    fn get_channel_fix_min(&mut self, channel: usize) -> crate::ControllerResult<u8> {
        if !self.has_channel(channel) {
            Err(ControllerError::NoSuchChannel(channel))
        } else {
            Ok(self.min[channel])
        }
    }

    fn get_channel_fix_mid(&mut self, channel: usize) -> crate::ControllerResult<f32> {
        if !self.has_channel(channel) {
            Err(ControllerError::NoSuchChannel(channel))
        } else {
            Ok(self.mid[channel])
        }
    }

    fn set_fix_type(&mut self, channel: usize, fix_type: FixType) -> crate::ControllerResult<()> {
        if !self.has_channel(channel) {
            Err(ControllerError::NoSuchChannel(channel))
        } else {
            self.fix_type[channel] = fix_type;
            Ok(())
        }
    }
}

impl<Device, Err, const CH: usize> BasicController<Device, Err, CH>
where
    Device: ReadData<Error = Err>,
    Err: Into<ControllerError>,
{
    const CHANNELS: usize = CH;
    pub fn new(device: Device) -> Self {
        Self {
            device,
            max: [u8::MAX; CH],
            min: [u8::MIN; CH],
            mid: [127.0; CH],
            last_updated: [u8::MIN; CH],
            p: PhantomData,
            fix_type: [FixType::None; CH],
        }
    }
}

#[cfg(feature = "hidapi")]
impl ReadData for hidapi::HidDevice {
    type Error = hidapi::HidError;
    fn read_data(&self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        self.read(buf)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ControllerResult;
    struct FakeDevice {
        result: ControllerResult<usize>,
        data: u8,
        add: bool,
    }

    const C: usize = 23;
    impl super::super::basic_controller::ReadData for FakeDevice {
        type Error = ControllerError;

        fn read_data(&self, buf: &mut [u8]) -> Result<usize, Self::Error> {
            let mut a = 0;
            for i in buf {
                *i = self.data + a;
                if self.add {
                    a += 1;
                }
            }
            match &self.result {
                Ok(n) => Ok(*n),
                Err(_e) => Err(ControllerError::NoSuchChannel(42)),
            }
        }
    }
    type TestType = super::super::basic_controller::BasicController<FakeDevice, ControllerError, C>;
    #[test]
    fn t1() {
        let mut a = TestType::new(FakeDevice {
            result: Ok(8),
            data: 223,
            add: false,
        });
        for i in &mut a.fix_type {
            *i = FixType::MaxMin;
        }
        assert_eq!(a.channels(), C);
        let _r = a.set_channel_fix(5, None, None, None).unwrap();
        assert_eq!(a.max, [u8::MAX; C]);
        assert_eq!(a.min, [u8::MIN; C]);
        assert_eq!(a.last_updated, [u8::MIN; C]);

        let _r = a.update().unwrap();
        assert_eq!(a.max, [u8::MAX; C]);
        assert_eq!(a.min, [u8::MIN; C]);
        assert_eq!(a.last_updated, [223; C]);

        assert_eq!(a.get_output_raw(1).unwrap(), 223);
        assert_eq!(a.get_output_raw(0).unwrap(), 223);
        assert!(a.get_output_raw(C).is_err());
        assert!(a.get_output_raw(C + 1).is_err());

        assert_eq!(a.get_output(1).unwrap(), 223);
        assert_eq!(a.get_output(0).unwrap(), 223);
        assert!(a.get_output(C).is_err());
        assert!(a.get_output(C + 1).is_err());

        assert_eq!(a.get_output_f32(1).unwrap(), 223.0 / 255.0);
        assert_eq!(a.get_output_f32(0).unwrap(), 223.0 / 255.0);
        assert!(a.get_output_f32(C).is_err());
        assert!(a.get_output_f32(C + 1).is_err());

        //////
        let _r = a.set_channel_fix(5, Some(240), Some(14), None).unwrap();
        assert_eq!(a.max, {
            let mut a = [u8::MAX; C];
            a[5] = 240;
            a
        });
        assert_eq!(a.min, {
            let mut a = [u8::MIN; C];
            a[5] = 14;
            a
        });
        assert_eq!(a.last_updated, [223; C]);

        assert_eq!(a.get_output_raw(1).unwrap(), 223);
        assert_eq!(a.get_output_raw(0).unwrap(), 223);
        assert!(a.get_output_raw(C).is_err());
        assert!(a.get_output_raw(C + 1).is_err());

        assert_eq!(a.get_output(1).unwrap(), 223);
        assert_eq!(a.get_output(0).unwrap(), 223);
        assert_eq!(
            a.get_output(5).unwrap(),
            ((223.0_f32 - 14.0) / (240.0 - 14.0) * 255.0).round() as u8
        );
        assert!(a.get_output(C).is_err());
        assert!(a.get_output(C + 1).is_err());

        assert_eq!(a.get_output_f32(1).unwrap(), 223.0 / 255.0);
        assert_eq!(a.get_output_f32(0).unwrap(), 223.0 / 255.0);
        assert_eq!(
            a.get_output_f32(5).unwrap(),
            ((223.0_f32 - 14.0) / (240.0 - 14.0))
        );
        assert!(a.get_output_f32(C).is_err());
        assert!(a.get_output_f32(C + 1).is_err());
    }
}
