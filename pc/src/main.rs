use log::{debug, info};
use m3_macro::include_map;
use m3_map::Map;
use macroquad::{prelude::*, window, Window};
use my_env_logger_style::TimestampPrecision;
use once_cell::sync::Lazy;

mod tiles;
use tiles::TEXTURES;
use usb::Players;

mod draw;
mod update;
mod usb;

const CARGO_PKG_NAME: &str = env!("CARGO_PKG_NAME");
const CARGO_PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

///store maps as String binary format
///call `Map::from_str()`
static LEVELS: Lazy<Vec<&str>> =
	Lazy::new(|| vec![include_map!("pc/assets/level/001.tmx")]);

struct GameState {
	level: Option<Map>,
	players: Players
}

impl GameState {
	fn new() -> GameState {
		Lazy::force(&TEXTURES);
		let level = Map::from_string(LEVELS.first().unwrap()).unwrap(); //tests check if map is vaild
		GameState {
			level: Some(level),
			players: usb::Players::init()
		}
	}
}

async fn run_game() {
	let mut game_state = GameState::new();
	loop {
		game_state.update().await;
		game_state.draw().await;
		next_frame().await
	}
}

fn main() {
	my_env_logger_style::set_timestamp_precision(TimestampPrecision::Disable);
	my_env_logger_style::just_log();
	info!("🚗 {CARGO_PKG_NAME}  v{CARGO_PKG_VERSION} 🚗");
	debug!("load level{:#?}", LEVELS[0]);
	Window::from_config(
		window::Conf {
			sample_count: 8, //anti-aliasing
			window_title: format!("{CARGO_PKG_NAME} v{CARGO_PKG_VERSION}"),
			high_dpi: true,
			#[cfg(not(debug_assertions))]
			fullscreen: true,
			..Default::default()
		},
		run_game()
	);
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn test_bundeld_maps() {
		for (i, map) in LEVELS.iter().enumerate() {
			//test if map can be deserialize
			Map::from_string(map).expect("map with index {i} is not valid");
		}
	}
}
