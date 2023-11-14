pub mod sdl_context;

use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::event::Event;

fn main() -> Result<(), String> {

    let screen_width = 800;
    let screen_height = 600;

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("Rust", screen_width, screen_height)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas()
        .build()
        .unwrap();

    let screen_area = Rect::new(0, 0, screen_width, screen_height);
    let clear_color = Color::RGB(64, 192, 255);



    let mut running = true;
    let mut event_queue = sdl_context.event_pump().unwrap();

    let mut RightArrowDown = false;
    let mut LeftArrowDown = false;
    let mut UpArrowDown = false;
    let mut DownArrowDown = false;

    let mut player_x: i32 = (screen_width / 2) as i32;
    let mut player_y: i32 = (screen_height / 2) as i32;
    let player_width = 60;
    let player_height = 60;
    let player_color = Color::RGB(255, 192, 0);
    let mut player_rect = Rect::new(player_x, player_y, player_width, player_height);
    let player_speed = 1;

    while running {

        // Events & Input
        for event in event_queue.poll_iter() {
            match event {
                Event::Quit {..} => {
                    running = false;
                },

                Event::MouseMotion { x, y, .. } => {
                    println!("Mouse x: {}, y: {}", x, y)
                }

                Event::KeyDown {keycode, ..} => {
                    if keycode.unwrap() == Keycode::Up { UpArrowDown = true; }
                    if keycode.unwrap() == Keycode::Down { DownArrowDown = true; }
                    if keycode.unwrap() == Keycode::Right { RightArrowDown = true; }
                    if keycode.unwrap() == Keycode::Left { LeftArrowDown = true; }
                },

                Event::KeyUp {keycode, ..} => {
                    if keycode.unwrap() == Keycode::Up { UpArrowDown = false; }
                    if keycode.unwrap() == Keycode::Down { DownArrowDown = false; }
                    if keycode.unwrap() == Keycode::Right { RightArrowDown = false; }
                    if keycode.unwrap() == Keycode::Left { LeftArrowDown = false; }
                }       
                _ => {}
            }
        }

        // Logic
        if UpArrowDown { player_y -= player_speed; }
        if DownArrowDown { player_y += player_speed; }
        if LeftArrowDown { player_x -= player_speed; }
        if RightArrowDown { player_x += player_speed; }

        if player_x < 0 { player_x = 0; }
        if player_y < 0 { player_y = 0; }

        if player_x > (screen_width - player_width) as i32 { player_x = (screen_width - player_width) as i32; }
        if player_y > (screen_height - player_height) as i32 { player_y = (screen_height - player_height) as i32; }

        player_rect.x = player_x;
        player_rect.y = player_y;

        // Rendering
        canvas.set_draw_color(clear_color);
        canvas.fill_rect(screen_area)?;

        canvas.set_draw_color(player_color);
        canvas.fill_rect(player_rect)?;

        canvas.present();
        
    }


    Ok(())
}