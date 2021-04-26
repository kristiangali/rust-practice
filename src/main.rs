// basic ggez template and setup (Use for ui testing along with bevy)
#![allow(unused_imports)]
#![allow(dead_code)]
pub mod utils;
use ggez::{Context, GameResult};
use utils::MainState;
use ggez::event;

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("input_test", "ggez");
    let (mut ctx, mut event_loop) = cb.build()?;

    let mut state = MainState::new();
    event::run(&mut ctx, &mut event_loop, &mut state)
}