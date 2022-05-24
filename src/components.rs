#![allow(dead_code)]

use crate::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerOwned;

#[derive(Component)]
pub struct Person;

#[derive(Component, Debug)]
pub struct Corporation {
    pub share_amount: u32,
    pub share_price: f64,
    pub share_holders: Vec<(Entity, u32)>
}
impl Corporation {
    pub fn new(share_amount: u32, share_price: f64, creator: Entity) -> Self {
        Self {
            share_amount,
            share_price,
            share_holders: vec![(creator, share_amount)]
        }
    }
}

#[derive(Component)]
pub struct Name(pub String);

#[derive(Component)]
pub struct Cash(pub f64);

#[derive(Component)]
pub struct OwnedShares(pub Vec<(Entity, u32)>);