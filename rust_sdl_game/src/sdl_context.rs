use sdl2::{Sdl, VideoSubsystem, video::Window, render::Canvas, EventPump};

pub struct SdlContext {
    pub sdl: Sdl,
    pub video_subsystem: VideoSubsystem,
    pub canvas: Canvas<Window>,
    pub event_queue: EventPump,
}

impl SdlContext {
    pub fn new(screen_width: u32, screen_height: u32) -> Result<SdlContext, String> {
        let sdl = sdl2::init()?;
        let video_subsystem = sdl.video()?;

        let window = video_subsystem.window("Rust", screen_width, screen_height)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas()
            .build()
            .unwrap();

        let event_queue = sdl.event_pump()?;
        
        let sdl_context: SdlContext = SdlContext { sdl, video_subsystem, canvas, event_queue };

        Ok(sdl_context)
    }
}

