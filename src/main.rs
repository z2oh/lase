use amethyst::{
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderDebugLines, RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::{
        application_root_dir,
        ortho_camera::CameraOrthoSystem,
    },
};

mod collisions;
mod components;
mod dodge;
mod resources;
mod systems;
mod util;
mod vector;

use crate::dodge::Dodge;

fn main() -> amethyst::Result<()> {
    // For now we log everything.
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    // Configuration files.
    let config_dir = app_root.join("config");
    let binding_path = config_dir.join("bindings.ron");
    let display_config_path = config_dir.join("display.ron");
    let laser_collision_config_path = config_dir.join("laser_collision.ron");
    let laser_spawner_config_path = config_dir.join("laser_spawner.ron");
    let time_scaling_config_path = config_dir.join("time_scaling.ron");

    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)
                        // Clear to black.
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderDebugLines::default()),
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with(
            CameraOrthoSystem::default(),
            "camera_system",
            &[],
        )
        .with(
            systems::PlayerSystem,
            "player_system",
            &["input_system"]
        )
        .with(
            // Explicit panic if an error is encountered while reading the
            // config file.
            systems::LaserSpawnerSystem::from_config_path(
                laser_spawner_config_path,
            ).unwrap(),
            "laser_system",
            &["player_system"]
        )
        .with(
            systems::RelativeMotionSystem,
            "relative_motion_system",
            &["player_system", "laser_system"]
        )
        .with(
            // Explicit panic if an error is encountered while reading the
            // config file.
            systems::LaserCollisionSystem::from_config_path(
                laser_collision_config_path,
            ).unwrap(),
            "laser_collision_system",
            // We want to check for collisions after everything has moved.
            &["relative_motion_system"]
        )
        .with(
            systems::TimeScalingSystem::from_config_path(
                time_scaling_config_path,
            ).unwrap(),
            "time_scaling_system",
            &["input_system"]
        )
        .with(
            systems::DebugSystem::default(),
            "debug_system",
            &[],
        );

    let assets_dir = app_root.join("assets");
    let mut game = Application::new(
        assets_dir,
        Dodge::with_config_path(config_dir),
        game_data,
    )?;
    game.run();

    Ok(())
}
