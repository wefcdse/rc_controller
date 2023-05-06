#![allow(unused)]
pub use super::error::ControllerError;
pub type ControllerResult<T> = Result<T, ControllerError>;

#[derive(Debug, Clone, Copy)]
pub enum FixType {
    MaxMin,
    MaxMidMin,
    None,
}
