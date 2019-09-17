use amethyst::core::Transform;
use amethyst::ecs::{Entities, Join, Read, ReadStorage, System, WriteStorage};
use amethyst::renderer::palette::Srgb;
use amethyst::renderer::resources::Tint;
use amethyst::renderer::SpriteRender;

use rand;

use crate::dodge::{Laser, Player, SpriteMap};

pub struct LaserSpawnerSystem {
    counter: u32,
    spawn_rate: u32,
}

impl Default for LaserSpawnerSystem {
    fn default() -> Self {
        Self {
            counter: 0,
            spawn_rate: 20,
        }
    }
}

impl<'s> System<'s> for LaserSpawnerSystem {
    type SystemData = (
        ReadStorage<'s, Player>,
        WriteStorage<'s, Laser>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, Tint>,
        Read<'s, SpriteMap>,
        Entities<'s>,
    );

    fn run(&mut self, (players, mut lasers, mut transforms, mut sprite_renderers, mut tints, sprite_map, entities): Self::SystemData) {
        self.counter += 1;
        if self.counter > self.spawn_rate {
            let player_transform = (&players, &transforms).join().next().map(|(_, t)| t).unwrap();
            let mut laser_transform = player_transform.clone();
            laser_transform.prepend_rotation_z_axis(1.0);
            let sprite_sheet = sprite_map.get("laser_sprite").unwrap();
            let sprite_renderer = SpriteRender {
                sprite_sheet,
                sprite_number: 0,
            };

            let laser_color = (rand::random(), rand::random(), rand::random());
            let laser = Laser {
                color: laser_color,
            };

            entities.build_entity()
                .with(sprite_renderer, &mut sprite_renderers)
                .with(laser_transform, &mut transforms)
                .with(laser, &mut lasers)
                .with(Tint(Srgb::from(laser_color).into()), &mut tints)
                .build();

            self.counter = 0;
        }
    }
}
