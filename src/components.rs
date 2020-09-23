use specs::prelude::*;
use specs_derive::Component;

#[derive(Component)]
pub struct Position {
    x: i32,
    y: i32,
}

#[derive(Component, Debug)]
struct Player {}

#[derive(Component, Debug)]
struct Box {}
