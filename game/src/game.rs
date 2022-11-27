use crate::resources::FERRIS_ICON;

use ::d2d::gfx::{target::RenderTarget, Color};
use ::geom::d2::{Dimension2D, Point2D};
use ::tracing::info;
use ::win32::{window::Window, *};
use ::windows::Win32::UI::WindowsAndMessaging::{
    DispatchMessageW, GetMessageW, PostQuitMessage, TranslateMessage, MSG,
};

pub struct Game {
    main_window: Window,
    window_title: String,

    render_target: RenderTarget,

    /// Dirty flag for changes that require rendering. If not dirty, we can skip
    /// rendering.
    is_render_dirty: bool,

    /// Tracks whether the main window is shutting down. If true, we should
    /// continue to pump winproc messages to finalize this process but we should
    /// avoid calling `update()`/`render()` or anything else that might interact
    /// with the window.
    is_shutting_down: bool,
}

impl Game {
    pub fn new() -> Self {
        let window_title = "Main Window".to_string();

        let dimension = Dimension2D {
            width: 800,
            height: 600,
        };
        let main_window = Window::new(dimension, &window_title, Some(FERRIS_ICON.id().into()))
            .expect("Failed to create main window");

        let render_target = RenderTarget::new_with_hwnd(main_window.hwnd(), dimension)
            .expect("Failed to create Direct2D render target");

        Self {
            main_window,
            window_title,
            render_target,
            is_render_dirty: false,
            is_shutting_down: false,
        }
    }

    fn update(&mut self) {
        let len = self.window_title.len();

        {
            let mut kbd = self.main_window.keyboard();
            let mut input = kbd.drain_input();
            self.window_title.truncate(
                self.window_title
                    .len()
                    .saturating_sub(input.num_backspaces()),
            );
            self.window_title.extend(input.chars());
        }

        if self.window_title.len() != len {
            self.is_render_dirty = true;
        }
    }

    fn draw(&mut self) {
        if !self.is_render_dirty {
            return;
        }

        let ctx = self.render_target.make_context();
        ctx.clear(Color::blue());
        ctx.put_pixel(Point2D { x: 10.0, y: 10.0 }, Color::red());

        self.main_window.set_title(&self.window_title).unwrap();
        self.is_render_dirty = false;
    }

    pub async fn run(&mut self) -> Result<()> {
        let mut msg = MSG::default();
        while unsafe { GetMessageW(&mut msg, None, 0, 0) }.as_bool() {
            unsafe { TranslateMessage(&msg) };
            unsafe { DispatchMessageW(&msg) };

            if self.main_window.clear_close_request() {
                info!("posting quit message");
                unsafe {
                    PostQuitMessage(0);
                }
                self.is_shutting_down = true;
            }

            if self.is_shutting_down {
                continue;
            }

            self.update();
            self.draw();
        }

        Ok(())
    }
}
