// #![deny(missing_docs)]
// #![deny(warnings)]
// #![no_std]


#[cfg(feature = "mosk")]
mod error;
#[cfg(feature = "mosk")]
pub use crate::error::MockError;

#[cfg(feature = "mosk")]
pub mod common;





pub use embedded_hal as hal;

mod relay;
mod valve;
mod sensor;
mod pump;
mod autosampler;
mod analog;
mod lamp;

pub use relay::Relay;
pub use valve::Valve;
pub use sensor::Sensor;
pub use pump::Pump;
pub use autosampler::Autosampler;
pub use analog::Analog;
pub use lamp::Lamp;