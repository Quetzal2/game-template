use crate::types::*;


// ///////////////////////////////////////////////////////////////////////
// Components
// ///////////////////////////////////////////////////////////////////////

/// A position in the game world.
#[derive(Clone, Debug)]
pub struct Position(pub Point2);

/// Motion in the game world.
#[derive(Clone, Debug)]
pub struct Motion {
    pub velocity: Vector2,
    pub acceleration: Vector2,
}

/// Just a marker that a particular entity is the player.
#[derive(Clone, Debug, Default)]
pub struct Player;

#[derive(Clone, Debug, Default)]
pub struct Shot {
    pub damage: u32,
}