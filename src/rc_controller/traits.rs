use crate::ControllerError;

use super::{ControllerResult, FixType};
pub trait Controller {
    fn channels(&self) -> usize;

    /// read data from physical device
    fn update(&mut self) -> ControllerResult<()>;

    fn get_output_raw(&self, channel: usize) -> ControllerResult<u8>;

    fn get_output(&self, channel: usize) -> ControllerResult<u8>;

    /// returns the channel's output in f32, min:0.0 max:1.0
    fn get_output_f32(&self, channel: usize) -> ControllerResult<f32>;

    fn set_channel_fix(
        &mut self,
        channel: usize,
        max: Option<u8>,
        min: Option<u8>,
        mid: Option<f32>,
    ) -> ControllerResult<()>;

    fn get_channel_fix_max(&mut self, channel: usize) -> ControllerResult<u8>;

    fn get_channel_fix_min(&mut self, channel: usize) -> ControllerResult<u8>;

    fn get_channel_fix_mid(&mut self, channel: usize) -> ControllerResult<f32>;

    fn set_fix_type(&mut self, channel: usize, fix_type: FixType) -> crate::ControllerResult<()>;
}

pub trait ControllerUtils {
    fn get_channel_result(&self, channel: usize) -> ControllerResult<usize>;
    fn has_channel(&self, channel: usize) -> bool;
    fn read_and_fix_f32_max_min(&mut self, channel: usize) -> ControllerResult<f32>;
    fn read_and_fix_f32_mid(&mut self, channel: usize, k: f32) -> ControllerResult<f32>;
    #[must_use]
    fn update_and_fix(&mut self, k: f32) -> ControllerResult<()>;
}
impl<C: Controller + ?Sized> ControllerUtils for C {
    fn has_channel(&self, channel: usize) -> bool {
        channel < self.channels()
    }
    fn get_channel_result(&self, channel: usize) -> ControllerResult<usize> {
        if self.has_channel(channel) {
            Ok(channel)
        } else {
            Err(ControllerError::NoSuchChannel(channel))
        }
    }

    fn read_and_fix_f32_max_min(&mut self, channel: usize) -> ControllerResult<f32> {
        let raw = self.get_output_raw(channel)?;
        let max_old = self.get_channel_fix_max(channel)?;
        let min_old = self.get_channel_fix_min(channel)?;
        self.set_channel_fix(
            channel,
            Some(u8::max(raw, max_old)),
            Some(u8::min(raw, min_old)),
            None,
        )?;
        self.get_output_f32(channel)
    }

    fn read_and_fix_f32_mid(&mut self, channel: usize, k: f32) -> ControllerResult<f32> {
        let raw = self.get_output_raw(channel)?;
        let mid_old = self.get_channel_fix_mid(channel)?;
        self.set_channel_fix(
            channel,
            None,
            None,
            Some(mid_old * (1.0 - k) + raw as f32 * k),
        )?;
        self.get_output_f32(channel)
    }

    fn update_and_fix(&mut self, k: f32) -> ControllerResult<()> {
        self.update()?;
        for channel in 0..self.channels() {
            let raw = self.get_output_raw(channel)?;
            let max_old = self.get_channel_fix_max(channel)?;
            let min_old = self.get_channel_fix_min(channel)?;
            let mid_old = self.get_channel_fix_mid(channel)?;
            self.set_channel_fix(
                channel,
                Some(u8::max(raw, max_old)),
                Some(u8::min(raw, min_old)),
                Some(mid_old * (1.0 - k) + raw as f32 * k),
            )?;
        }
        Ok(())
    }
}
