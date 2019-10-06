//! This component holds properties about a player.
/// The `PlayerConfig` holds certain configurable properties about a player,
/// namely properties related to physics.
// TODO: hopefully remove the `Default` derivation pending this issue:
// https://github.com/amethyst/amethyst/issues/1954
use std::path::Path;

use amethyst::config::{Config, ConfigError};
use amethyst::ecs::prelude::{Component, DenseVecStorage};

use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize)]
pub struct PlayerConfig {
    pub max_speed: f32,
    pub acceleration: f32,
    pub deceleration: f32,
}

// TODO: is this idiomatic for singleton entities like the player?
pub struct Player {
    pub config: PlayerConfig,
}

impl Player {
    pub fn from_config(config: impl Into<PlayerConfig>) -> Self {
        Self {
            config: config.into(),
        }
    }

    /// Builds a `Player` by reading the RON file at `path`.
    pub fn from_config_path(
        path: impl AsRef<Path>
    ) -> Result<Self, ConfigError> {
        // TODO: hopefully change this to just call load pending this issue:
        // https://github.com/amethyst/amethyst/issues/1954
        PlayerConfig::load_no_fallback(path).map(Self::from_config)
    }
}

impl Component for Player {
    // TODO: investigate storage types.
    type Storage = DenseVecStorage<Self>;
}
