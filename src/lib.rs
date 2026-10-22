use rust_embed::RustEmbed;

pub mod cli;
pub mod list;
pub mod pokemon;
pub mod sprites;

#[derive(RustEmbed)]
#[folder = "data/pokesprite/pokemon"]
pub struct Data;
