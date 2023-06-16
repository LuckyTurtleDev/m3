use crate::{update::init_level, Activity, GameState, Phase, LEVELS};
use m3_models::ToPcGameEvent;
use macroquad::{
	hash,
	prelude::*,
	ui::{root_ui, widgets::Button, Skin}
};

const BUTTON_FONT_SIZE: u16 = 16;

fn get_menu_skin() -> Skin {
	{
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
				include_bytes!("../assets/img/Menu/button_hovered_background.png"),
				None
			))
			.background_clicked(Image::from_file_with_format(
				include_bytes!("../assets/img/Menu/button_clicked_background.png"),
				None
			))
			.text_color(Color::from_rgba(180, 180, 100, 255))
			.font_size(BUTTON_FONT_SIZE)
			.build();
		Skin {
			window_style,
			button_style,
			..root_ui().default_skin()
		}
	}
}

impl GameState {
	pub(crate) async fn build_menu(&mut self, events: &[Option<Vec<ToPcGameEvent>>; 4]) {
		clear_background(GRAY);
		let screen_width = screen_width();
		let screen_height = screen_height();
		let menu_size = vec2(screen_width * 0.5, screen_height * 0.8);
		let menu_position = vec2(
			(screen_width - menu_size.x) / 2.0,
			(screen_height - menu_size.y) / 2.0
		);
		let menu_skin = get_menu_skin();
		debug!("Menu loop");
		root_ui().push_skin(&menu_skin);
		while self.activity == Activity::Menu {
			clear_background(GRAY);
			root_ui().window(hash!(), menu_position, menu_size, |ui| {
				let play_button_id = hash!("play_button");
				let play_button_text = "Play";
				let play_button_text_dim =
					measure_text(play_button_text, None, BUTTON_FONT_SIZE, 1.0);

				let play_button = Button::new("Play")
                .position(vec2(200.0-60.0, 60.0))
				//.size(vec2(play_button_text_width + 20.0, 30.0))
				//.selected(true)
                .ui(ui);

				if play_button {
					debug!("Play pressed");
					self.activity = Activity::SelectLevel;
				}
				if ui.button(None, "Quit") {
					self.running = false;
				}
			});
			next_frame().await;
		}
	}

	pub(crate) async fn build_level_menu(
		&mut self,
		events: &[Option<Vec<ToPcGameEvent>>; 4]
	) {
		clear_background(GRAY);
		let screen_width = screen_width();
		let screen_height = screen_height();
		let menu_size = vec2(screen_width * 0.5, screen_height * 0.8);
		let menu_position = vec2(
			(screen_width - menu_size.x) / 2.0,
			(screen_height - menu_size.y) / 2.0
		);
		let menu_skin = get_menu_skin();
		root_ui().push_skin(&menu_skin);
		while self.activity == Activity::SelectLevel {
			root_ui().window(hash!(), menu_position, menu_size, |ui| {
				if ui.button(None, "Tutorial") {
					debug!("Play pressed");
					todo!("Tutorial");
				}
				for x in 0..LEVELS.len() {
					if ui.button(None, format!("Level {}", x + 1)) {
						self.level_num = x;
						init_level(self);
						self.activity = Activity::GameRound(Phase::Select);
					}
				}
				if ui.button(None, "Quit") {
					self.running = false;
				}
			});
			next_frame().await;
		}
	}
}