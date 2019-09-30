/// Wrapper around a float that controls how fast time within the game is
/// moving. This is a multiplicative factor.
pub struct TimeScale(pub f32);
impl Default for TimeScale {
    /// The default timescale is 1.0, so time will move at the standard speed.
    fn default() -> Self {
        Self(1.0)
    }
}
