use rule_30_sdl::{App, State};
use sdl3::event::Event;
use sdl3::{self, pixels::PixelFormat};

pub const TITLE: &str = "Rule 30";
pub const WIDTH: u32 = 800;
pub const HEIGHT: u32 = 800;

fn setup() -> App {
    App::new(WIDTH, HEIGHT)
}

fn main() {
    let program_start = std::time::Instant::now();

    let mut app = setup();

    let sdl = sdl3::init().unwrap();
    let video = sdl.video().unwrap();
    let mut event_pump = sdl.event_pump().unwrap();

    let window = video
        .window(TITLE, WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas();
    let texture_creator = canvas.texture_creator();
    let mut app_texture = texture_creator
        .create_texture_streaming(PixelFormat::RGB24, WIDTH, HEIGHT)
        .unwrap();

    'running: loop {
        let frame_start = std::time::Instant::now();

        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                _ => (),
            }
        }

        app.next();
        app.update_frame_buffer();

        // Here, we are going to copy over the frame buffer data into the image
        // to be drawn to the canvas
        let pixels = app.frame_buffer().to_bytes();
        app_texture
            .update(None, &pixels[..], WIDTH as usize * 3)
            .unwrap();
        canvas.copy(&app_texture, None, None).unwrap();

        canvas.present();

        println!("{} ms", frame_start.elapsed().as_millis());
    }
}
