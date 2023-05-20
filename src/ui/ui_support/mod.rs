use glium::glutin;
use glium::glutin::event::{ElementState, Event, MouseButton, WindowEvent};
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use glium::glutin::window::WindowBuilder;
use glium::Display;
use imgui::{Context, FontConfig, FontGlyphRanges, FontSource};
use imgui_glium_renderer::Renderer;
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use std::path::Path;
use std::time::Instant;

mod clipboard;

pub struct AppSettings {
    pub window_width: f64,
    pub window_height: f64,
    pub window_name: String,
    pub max_fps: f32,
}

pub struct System {
    pub event_loop: EventLoop<()>,
    pub display: glium::Display,
    pub imgui: Context,
    pub platform: WinitPlatform,
    pub renderer: Renderer,
    pub font_size: f32,
    max_fps: f32,
}

pub fn init(settings: &AppSettings) -> System {
    let title = match Path::new(&settings.window_name).file_name() {
        Some(file_name) => file_name.to_str().unwrap(),
        None => &settings.window_name,
    };
    let event_loop = EventLoop::new();
    let context = glutin::ContextBuilder::new().with_vsync(true);
    let builder = WindowBuilder::new()
        .with_title(title.to_owned())
        .with_inner_size(glutin::dpi::LogicalSize::new(
            settings.window_width,
            settings.window_height,
        ))
        .with_resizable(false);
    let display =
        Display::new(builder, context, &event_loop).expect("Failed to initialize display");

    let mut imgui = Context::create();
    imgui.set_ini_filename(None);

    if let Some(backend) = clipboard::init() {
        imgui.set_clipboard_backend(backend);
    } else {
        eprintln!("Failed to initialize clipboard");
    }

    let mut platform = WinitPlatform::init(&mut imgui);
    {
        let gl_window = display.gl_window();
        let window = gl_window.window();

        let dpi_mode = if let Ok(factor) = std::env::var("IMGUI_EXAMPLE_FORCE_DPI_FACTOR") {
            // Allow forcing of HiDPI factor for debugging purposes
            match factor.parse::<f64>() {
                Ok(f) => HiDpiMode::Locked(f),
                Err(e) => panic!("Invalid scaling factor: {}", e),
            }
        } else {
            HiDpiMode::Default
        };

        platform.attach_window(imgui.io_mut(), window, dpi_mode);
    }

    // Fixed font size. Note imgui_winit_support uses "logical
    // pixels", which are physical pixels scaled by the devices
    // scaling factor. Meaning, 13.0 pixels should look the same size
    // on two different screens, and thus we do not need to scale this
    // value (as the scaling is handled by winit)
    let font_size = 13.0;

    imgui.fonts().add_font(&[
        FontSource::TtfData {
            data: include_bytes!("../resources/Roboto-Regular.ttf"),
            size_pixels: font_size,
            config: Some(FontConfig {
                // As imgui-glium-renderer isn't gamma-correct with
                // it's font rendering, we apply an arbitrary
                // multiplier to make the font a bit "heavier". With
                // default imgui-glow-renderer this is unnecessary.
                rasterizer_multiply: 1.5,
                // Oversampling font helps improve text rendering at
                // expense of larger font atlas texture.
                oversample_h: 4,
                oversample_v: 4,
                ..FontConfig::default()
            }),
        },
        FontSource::TtfData {
            data: include_bytes!("../resources/mplus-1p-regular.ttf"),
            size_pixels: font_size,
            config: Some(FontConfig {
                // Oversampling font helps improve text rendering at
                // expense of larger font atlas texture.
                oversample_h: 4,
                oversample_v: 4,
                // Range of glyphs to rasterize
                glyph_ranges: FontGlyphRanges::japanese(),
                ..FontConfig::default()
            }),
        },
    ]);

    let renderer = Renderer::init(&mut imgui, &display).expect("Failed to initialize renderer");

    System {
        event_loop,
        display,
        imgui,
        platform,
        renderer,
        font_size,
        max_fps: settings.max_fps,
    }
}

impl System {
    pub fn main_loop<F: FnMut(&mut bool, &Display, &(f64, f64), &ElementState) + 'static>(
        self,
        mut run_ui: F,
    ) {
        let System {
            event_loop,
            display,
            mut imgui,
            mut platform,
            ..
        } = self;
        let mut last_frame = Instant::now();
        let mut prev_frame_t = Instant::now();

        let mut cursor_position: (f64, f64) = (0.0, 0.0);
        let mut mouse_input_state: ElementState = ElementState::Released;

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            match event {
                Event::NewEvents(_) => {
                    let now = Instant::now();
                    imgui.io_mut().update_delta_time(now - last_frame);
                    last_frame = now;
                }
                Event::MainEventsCleared => {
                    let gl_window = display.gl_window();
                    platform
                        .prepare_frame(imgui.io_mut(), gl_window.window())
                        .expect("Failed to prepare frame");
                    gl_window.window().request_redraw();
                }
                Event::RedrawRequested(_) => {
                    let redraw_request_t = Instant::now();
                    if (redraw_request_t - prev_frame_t).as_secs_f32() > 1. / self.max_fps as f32 {
                        let mut run = true;
                        run_ui(&mut run, &display, &cursor_position, &mouse_input_state);
                        if !run {
                            *control_flow = ControlFlow::Exit;
                        }
                        prev_frame_t = Instant::now();
                    }
                }
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        println!("Received termination signal.");
                        *control_flow = glutin::event_loop::ControlFlow::Exit;
                        return;
                    }
                    WindowEvent::CursorMoved { position, .. } => {
                        cursor_position = (position.x, position.y);
                    }
                    WindowEvent::MouseInput {
                        device_id: _,
                        state,
                        button,
                        modifiers: _,
                    } => {
                        if button == MouseButton::Left {
                            mouse_input_state = state;
                        }
                    }
                    _ => return,
                },
                event => {
                    let gl_window = display.gl_window();
                    platform.handle_event(imgui.io_mut(), gl_window.window(), &event);
                }
            }
        })
    }
}
