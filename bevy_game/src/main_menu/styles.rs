use bevy::prelude::*;

pub const NORMAL_BUTTON_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON_COLOR: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON_COLOR: Color = Color::rgb(0.75, 0.75, 0.75);

pub fn get_button_style() -> Style {
    Style {
        width: Val::Px(200.0),
        height: Val::Px(80.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        margin: UiRect::new(
            Val::Px(0.0),
            Val::Px(0.0),
            Val::Px(8.0),
            Val::Px(8.0)
        ),
        ..Style::DEFAULT
    }
}

pub fn get_image_style() -> Style {
    Style {
        width: Val::Px(64.0),
        height: Val::Px(64.0),
        margin: UiRect::new(
            Val::Px(8.0),
            Val::Px(8.0),
            Val::Px(8.0),
            Val::Px(8.0),
        ),
        ..Style::DEFAULT
    }
}

pub fn get_default_text(
    asset_server: &Res<AssetServer>,
    text: &str,
    font_size: f32
) -> Text {
    Text {
        sections: vec![
            TextSection::new(
                text,
                TextStyle { 
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"), 
                    font_size: font_size, 
                    color: Color::WHITE
                }
            )
        ],
        alignment: TextAlignment::Center,
        ..Text::default()
    }
}