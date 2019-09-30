use std::iter::FromIterator;
use std::collections::HashMap;

use amethyst::{
    assets::Handle,
    renderer::SpriteSheet,
};

/// Holds a hash map mapping string ids to a sprite sheet handle. This allows
/// easy look up of sprites in systems.
// TODO: Make this type checked by having some kind of enum system for hardcoded
// texture ids?
// TODO: Generalize this to work with generic asset types.
#[derive(Default)]
pub struct SpriteMap(HashMap<String, Handle<SpriteSheet>>);
// TODO: Evaluate this public API.
impl SpriteMap {
    /// Inserts a new handle with its string id into the map.
    #[allow(dead_code)]
    pub fn insert(
        &mut self,
        k: String,
        v: Handle<SpriteSheet>
    ) -> Option<Handle<SpriteSheet>> {
        self.0.insert(k, v)
    }

    /// Gets the handle to the sprite sheet. Since `Handle` clones are cheap, we
    /// clone here so that the caller doesn't have to.
    pub fn get(&self, k: &str) -> Option<Handle<SpriteSheet>> {
        self.0.get(k).map(Clone::clone)
    }
}

impl<K, V> FromIterator<(K, V)> for SpriteMap
where
    K: Into<String>,
    V: Into<Handle<SpriteSheet>>,
{
    /// Builds a `SpriteMap` from any arity-two tuple in which the first element
    /// can be converted into a `String` and the second element can be converted
    /// into a `Handle<SpriteSheet>`. It is expected this this function will
    /// only be called via the `iter::Iterator::collect` function.
    ///
    /// This function will allocate.
    ///
    /// # Examples
    /// ```rust
    /// // Useful for testing; when asset names can be hardcoded at compile
    /// time.
    /// [
    ///     ("laser_sprite", laser_sprite_sheet_handle),
    ///     ("player", sprites_sprite_sheet_handle),
    /// ].into_iter().cloned().collect()
    /// ```
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> SpriteMap {
        SpriteMap(iter.into_iter().map(|(k, v)| (k.into(), v.into())).collect())
    }
}
