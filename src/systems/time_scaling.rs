//! This module updates the current time scaling factor as a function of how
//! long the player has been holding the input. As the player provides input,
//! the time scaling factor is increased (meaning time speeds up). After input
//! is no longer being provided, the time scaling factor is decreased. The value
//! is clamped between a configurable maximum and minium time scaling value.
use std::path::Path;

use amethyst::config::{Config, ConfigError};
use amethyst::core::Time;
use amethyst::ecs::{Read, System, Write};
use amethyst::input::{InputHandler, StringBindings};

use serde::{Deserialize, Serialize};

use crate::resources::TimeScale;
use crate::util::prelude::*;

// TODO: hopefully remove the `Default` derivation pending this issue:
// https://github.com/amethyst/amethyst/issues/1954
#[derive(Default, Deserialize, Serialize)]
pub struct TimeScalingConfig {
    /// The minimum allowed timescale.
    minimum_time_scale: f32,
    /// The maximum allowed timescale.
    maximum_time_scale: f32,
    /// The factor by which time scaling in the positive direction is scaled.
    time_scale_positive_scaling_factor: f32,
    /// The factor by which time scaling in the negative direction is scaled.
    time_scale_negative_scaling_factor: f32,
}

pub struct TimeScalingSystem {
    config: TimeScalingConfig,
}

impl TimeScalingSystem {
    /// Builds a `TimeScalingSystem` with the provided `TimeScalingConfig`.
    pub fn from_config(config: impl Into<TimeScalingConfig>) -> Self {
        Self {
            config: config.into(),
        }
    }

    /// Builds a `TimeScalingSystem` by reading the RON file at `path`.
    pub fn from_config_path(
        path: impl AsRef<Path>
    ) -> Result<Self, ConfigError> {
        // TODO: hopefully change this to just call load pending this issue:
        // https://github.com/amethyst/amethyst/issues/1954
        TimeScalingConfig::load_no_fallback(path).map(Self::from_config)
    }
}

impl<'s> System<'s> for TimeScalingSystem {
    // TODO: is this idiomatic? Can I package these in some more convenient
    // structure?
    type SystemData = (
        Read<'s, InputHandler<StringBindings>>,
        Write<'s, TimeScale>,
        Read<'s, Time>,
    );

    fn run(
        &mut self,
        (
            input,
            mut time_scale,
            time
        ): Self::SystemData
    ) {
        let config = &self.config;
        // Grab the raw input values.
        let x_in = input.axis_value("x_in").unwrap_or(0.0);
        let y_in = input.axis_value("y_in").unwrap_or(0.0);

        let time_scale_delta = if x_in == 0.0 && y_in == 0.0 {
            config.time_scale_negative_scaling_factor * time.delta_seconds()
        } else {
            config.time_scale_positive_scaling_factor * time.delta_seconds()
        };

        time_scale.0 = clamp(
            time_scale.0 + time_scale_delta,
            self.config.minimum_time_scale,
            self.config.maximum_time_scale,
        );
    }
}
