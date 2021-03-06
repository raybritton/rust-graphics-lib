use std::thread::sleep;
use std::time::Duration;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit_input_helper::WinitInputHelper;
use pixels_graphics_lib::color::{BLACK, BLUE, Color, CYAN, GREEN, MAGENTA, RED, WHITE, YELLOW};
use pixels_graphics_lib::drawing::PixelWrapper;
use pixels_graphics_lib::{setup, WindowScaling};
use anyhow::Result;
use pixels_graphics_lib::prefs::WindowPreferences;
use pixels_graphics_lib::text::TextSize;

/// Running this example will create preference directories and files on your computer!
///
/// This example shows how to use `WindowPreferences` to save and restore the window size and position
/// It also has a small text demo

fn main() -> Result<()> {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let (mut window, mut graphics) = setup((240, 160), WindowScaling::AutoFixed(2), "Window Position Example", &event_loop)?;
    let mut prefs = WindowPreferences::new("app", "pixels-graphics-lib-example", "window-pos2")?;
    prefs.load()?;
    prefs.restore(&mut window);

    let mut scene = WindowPrefsScene::new(&mut graphics, "Example text", 240, 160);

    event_loop.run(move |event, _, control_flow| {
        if let Event::LoopDestroyed = event {
            prefs.store(&window);
            //can't return from here so just print out error
            let _result = prefs.save()
                .map_err(|err| eprintln!("Unable to save prefs: {:?}", err));
        }

        if let Event::RedrawRequested(_) = event {
            if graphics.pixels
                .render()
                .map_err( | e | eprintln ! ("pixels.render() failed: {:?}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }

            scene.render(&mut graphics);
        }

        scene.update();

        if input.update( & event) {
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if let Some(size) = input.window_resized() {
                graphics.pixels.resize_surface(size.width, size.height);
            }

            //put your input handling code here

            window.request_redraw();
        }

        sleep(Duration::from_millis(30));
    });
}

struct WindowPrefsScene {
    text: &'static str,
    pos: (usize, usize),
    colors: Vec<Color>,
    idx: usize
}

impl WindowPrefsScene {
    pub fn new(graphics: &mut PixelWrapper, text: &'static str, width: usize, height: usize) -> Self {
        let (w, h) = graphics.get_text_size(&text, 12, TextSize::Normal);
        let pos = (width / 2 - w / 2, height / 2 - h / 2);
        WindowPrefsScene { text, pos, colors: vec![GREEN, RED, BLUE, WHITE, MAGENTA, YELLOW, CYAN], idx: 0 }
    }
}

impl WindowPrefsScene {
    fn update(&mut self) {
        if self.idx < self.colors.len() - 1 {
            self.idx += 1;
        } else {
            self.idx = 0;
        }
    }

    fn render(&self, graphics: &mut PixelWrapper) {
        graphics.clear(BLACK);
        let mut color_idx = self.idx;
        for (i, letter) in self.text.chars().enumerate() {
            let mut pos = self.pos;
            pos.0 += TextSize::Normal.get_size().0 * i + TextSize::Normal.get_margin() * i;
            graphics.draw_letter_px(pos.0 as isize, pos.1 as isize, letter, TextSize::Normal, self.colors[color_idx]);

            color_idx += 1;
            if color_idx >= self.colors.len() {
                color_idx = 0;
            }
        }
    }
}