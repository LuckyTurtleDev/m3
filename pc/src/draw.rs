use crate::{
	tiles::GetTexture, Activity, GameState, Orientation, Phase, Rotation, TEXTURES
};
use macroquad::{
	hash,
	math::Vec2,
	prelude::*,
	ui::{root_ui, widgets::Button, Skin}
};

fn draw_menu(games_state: &mut GameState) {
	clear_background(GRAY);
	let screen_width = screen_width();
	let screen_height = screen_height();
	let menu_size = vec2(screen_width * 0.5, screen_height * 0.8);
	let menu_position = vec2(
		(screen_width - menu_size.x) / 2.0,
		(screen_height - menu_size.y) / 2.0
	);
	let button_font_size = 40;
	let menu_skin = {
		let window_style = root_ui()
			.style_builder()
			.background(Image::from_file_with_format(
				include_bytes!("../assets/img/Menu/selection_box_background.png"),
				None
			))
			.background_margin(RectOffset::new(20.0, 20.0, 10.0, 10.0))
			.margin(RectOffset::new(-20.0, -30.0, 0.0, 0.0))
			.build();
		let button_style = root_ui()
			.style_builder()
			.background(Image::from_file_with_format(
				include_bytes!("../assets/img/Menu/button_background.png"),
				None
			))
			.background_margin(RectOffset::new(37.0, 37.0, 5.0, 5.0))
			.margin(RectOffset::new(10.0, 10.0, 0.0, 0.0))
			.background_hovered(Image::from_file_with_format(
				include_bytes!("../assets/img/Menu/button_background.png"),
				None
			))
			.background_clicked(Image::from_file_with_format(
				include_bytes!("../assets/img/Menu/button_background.png"),
				None
			))
			.text_color(Color::from_rgba(180, 180, 100, 255))
			.font_size(button_font_size)
			.build();
		Skin {
			window_style,
			button_style,
			..root_ui().default_skin()
		}
	};
	root_ui().push_skin(&menu_skin);
	root_ui().window(hash!(), menu_position, menu_size, |ui| {
		if ui.button(None, "Play") {
			debug!("Play pressed");
			games_state.activity = Activity::GameRound(Phase::Select);
		}
	});
	//widgets::Window::new(hash!(), vec2(300.0, 300.0), vec2(100.0, 100.0)).label("Menu");
}

impl GameState {
	///draw the menu

	///draw the current game state
	pub(crate) async fn draw(&mut self) {
		clear_background(BLACK);
		let screen_width = screen_width();
		let screen_height = screen_height();
		match &self.activity {
			crate::Activity::Menu => {
				draw_menu(self); //self.draw_menu(&self.input_players, screen_width, screen_height);
			},
			crate::Activity::GameRound(_) => {
				match &self.game_run {
					None => todo!(),
					Some(game_run) => {
						//draw map
						let dest_size = (screen_width / game_run.level.width as f32)
							.min(screen_height / game_run.level.height as f32);
						//center map, by using offset
						let map_offset_x = (screen_width
							- dest_size * game_run.level.width as f32)
							/ 2.0;
						let map_offset_y = (screen_height
							- dest_size * game_run.level.height as f32)
							/ 2.0;
						for (x, y, tile) in game_run.level.iter_all() {
							let texture = tile.texture(&TEXTURES);
							let draw_params = DrawTextureParams {
								dest_size: Some(Vec2::new(dest_size, dest_size)),
								..Default::default()
							};
							draw_texture_ex(
								texture,
								x as f32 * dest_size + map_offset_x,
								y as f32 * dest_size + map_offset_y,
								//This param can filter colors.
								//Set every value to 1 to keep all colors, by using WHITE
								WHITE,
								draw_params
							);
						}

						//draw players
						let player_textures = TEXTURES.get_player_textures();
						for (x, player) in game_run.level.iter_player().enumerate() {
							if self.input_players.players[x].is_some()
								&& !game_run.player_states[x].finished
							{
								let texture = player_textures[x];
								// Car drives forward
								if game_run.player_states[x].rotation
									== Rotation::NoRotation
								{
									let relative_pos_x =
										(game_run.player_states[x].position.0 as f32
											- player.position.0 as f32) * dest_size / (self
											.movement_time
											/ self.delta_time);
									let relative_pos_y =
										(game_run.player_states[x].position.1 as f32
											- player.position.1 as f32) * dest_size / (self
											.movement_time
											/ self.delta_time);
									let rotation: f32 = match player.orientation {
										Orientation::North => 0.0,
										Orientation::East => 90.0,
										Orientation::South => 180.0,
										Orientation::West => 270.0
									};
									let draw_params = DrawTextureParams {
										dest_size: Some(Vec2::new(dest_size, dest_size)),
										rotation: rotation.to_radians(),
										..Default::default()
									};
									draw_texture_ex(
										texture,
										player.position.0 as f32 * dest_size
											+ relative_pos_x + map_offset_x,
										player.position.1 as f32 * dest_size
											+ relative_pos_y + map_offset_y,
										WHITE,
										draw_params
									);
								// Car makes a turn
								} else {
									let sign = match game_run.player_states[x].rotation {
										Rotation::RotateLeft => -1.0,
										Rotation::RotateRight => 1.0,
										_ => unreachable!()
									};
									let angle_offset = match player.orientation {
										Orientation::North => 0.0,
										Orientation::East => 90.0,
										Orientation::South => 180.0,
										Orientation::West => 270.0
									};
									let angle: f32 = (90.0
										/ (self.movement_time / self.delta_time))
										* sign + angle_offset;
									let draw_params = DrawTextureParams {
										dest_size: Some(Vec2::new(dest_size, dest_size)),
										rotation: angle.to_radians(),
										..Default::default()
									};
									draw_texture_ex(
										texture,
										player.position.0 as f32 * dest_size
											+ map_offset_x,
										player.position.1 as f32 * dest_size
											+ map_offset_y,
										WHITE,
										draw_params
									);
								}
							}
						}
					}
				}
			}
		}
	}
}
