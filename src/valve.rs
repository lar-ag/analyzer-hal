/// Valve I/O

/// Single digital push-pull output pin
pub trait Valve {
    /// Error type
    type Error;

    /// Drives the pin low
    ///
    /// *NOTE* the actual electrical state of the pin may not actually be low, e.g. due to external
    /// electrical sources
    fn open(&mut self) -> Result<(), Self::Error>;

    /// Drives the pin high
    ///
    /// *NOTE* the actual electrical state of the pin may not actually be high, e.g. due to external
    /// electrical sources
    fn close(&mut self) -> Result<(), Self::Error>;

}
