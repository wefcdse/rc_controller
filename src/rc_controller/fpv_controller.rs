use super::Controller;
use super::ControllerError;
use super::ControllerResult;
use super::ControllerUtils;
use super::FixType;
pub struct BasicFPVController<C: Controller> {
    throttle: Option<usize>, // 油门
    yaw: Option<usize>,      // 偏航
    pitch: Option<usize>,    // 俯仰
    roll: Option<usize>,     // 翻滚
    controller: C,
    initiallized: bool,
}

impl<C: Controller> BasicFPVController<C> {
    pub fn new(controller: C) -> Self {
        Self {
            throttle: None,
            yaw: None,
            pitch: None,
            roll: None,
            controller,
            initiallized: false,
        }
    }
    pub fn set_channels(
        &mut self,
        throttle: Option<usize>, // 油门
        yaw: Option<usize>,      // 偏航
        pitch: Option<usize>,    // 俯仰
        roll: Option<usize>,     // 翻滚
    ) -> ControllerResult<()> {
        match throttle {
            Some(v) => {
                let v = self.controller.get_channel_result(v)?;
                self.throttle = Some(v);
                self.initiallized = false;
            }
            None => {}
        }
        match yaw {
            Some(v) => {
                let v = self.controller.get_channel_result(v)?;
                self.yaw = Some(v);
                self.initiallized = false;
            }
            None => {}
        }
        match pitch {
            Some(v) => {
                let v = self.controller.get_channel_result(v)?;
                self.pitch = Some(v);
                self.initiallized = false;
            }
            None => {}
        }
        match roll {
            Some(v) => {
                let v = self.controller.get_channel_result(v)?;
                self.roll = Some(v);
                self.initiallized = false;
            }
            None => {}
        }

        Ok(())
    }

    pub fn init(&mut self) -> ControllerResult<()> {
        match self.throttle {
            Some(v) => {
                self.controller.set_fix_type(v, FixType::MaxMin)?;
            }
            None => return Err(ControllerError::LeverNotAsigned(String::from("throttle"))),
        }
        match self.yaw {
            Some(v) => {
                self.controller.set_fix_type(v, FixType::MaxMidMin)?;
            }
            None => return Err(ControllerError::LeverNotAsigned(String::from("yaw"))),
        }
        match self.pitch {
            Some(v) => {
                self.controller.set_fix_type(v, FixType::MaxMidMin)?;
            }
            None => return Err(ControllerError::LeverNotAsigned(String::from("pitch"))),
        }
        match self.roll {
            Some(v) => {
                self.controller.set_fix_type(v, FixType::MaxMidMin)?;
            }
            None => return Err(ControllerError::LeverNotAsigned(String::from("roll"))),
        }

        self.initiallized = true;
        Ok(())
    }

    pub fn update(&mut self) -> ControllerResult<()> {
        self.controller.update()
    }

    pub fn get_throttle(&self) -> ControllerResult<f32> {
        if !self.initiallized {
            return Err(ControllerError::NotInitiallized);
        }
        let c = match self.throttle {
            Some(v) => v,
            None => {
                return Err(ControllerError::LeverNotAsigned("throttle".to_string()));
            }
        };
        let v = self.controller.get_output_f32(c)?;
        Ok(v)
    }

    pub fn get_yaw(&self) -> ControllerResult<f32> {
        if !self.initiallized {
            return Err(ControllerError::NotInitiallized);
        }
        let c = match self.yaw {
            Some(v) => v,
            None => {
                return Err(ControllerError::LeverNotAsigned("yaw".to_string()));
            }
        };
        let v = self.controller.get_output_f32(c)?;
        Ok(v * 2.0 - 1.0)
    }
    pub fn get_pitch(&self) -> ControllerResult<f32> {
        if !self.initiallized {
            return Err(ControllerError::NotInitiallized);
        }
        let c = match self.pitch {
            Some(v) => v,
            None => {
                return Err(ControllerError::LeverNotAsigned("pitch".to_string()));
            }
        };
        let v = self.controller.get_output_f32(c)?;
        Ok(v * 2.0 - 1.0)
    }
    pub fn get_roll(&self) -> ControllerResult<f32> {
        if !self.initiallized {
            return Err(ControllerError::NotInitiallized);
        }
        let c = match self.roll {
            Some(v) => v,
            None => {
                return Err(ControllerError::LeverNotAsigned("roll".to_string()));
            }
        };
        let v = self.controller.get_output_f32(c)?;
        Ok(v * 2.0 - 1.0)
    }

    /// throttle, yaw, pitch, roll
    pub fn get_typr(&self) -> ControllerResult<(f32, f32, f32, f32)> {
        Ok((
            self.get_throttle()?,
            self.get_yaw()?,
            self.get_pitch()?,
            self.get_roll()?,
        ))
    }
}
