#![deny(clippy::all)]
#![forbid(unsafe_code)]

mod complex_plane;
use crate::complex_plane::ComplexPlane;



use winit::dpi::{LogicalPosition, LogicalSize, PhysicalSize};

use winit::event::VirtualKeyCode;
use winit::event_loop::{EventLoop};

use num_complex::Complex;
use rayon::prelude::*;
use hsl::HSL;

use glium::{Surface, implement_vertex, uniform};
use glium::backend::{Backend, Facade};

#[allow(unused_imports)]
use glium::glutin;

const SCREEN_WIDTH: u32 = 1280;
const SCREEN_HEIGHT: u32 = 800;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

struct Context {
    display: glium::Display,
    program: glium::Program,
    vertex_buffer: glium::VertexBuffer<Vertex>,
}

impl Context {
    fn new(display: glium::Display, program: glium::Program, vertex_buffer: glium::VertexBuffer<Vertex>) -> Self {
        return Context {
            display,
            program,
            vertex_buffer
        }
    }
    fn redraw(&self, max_iterations: i32, complex_plane: ComplexPlane, pixel_size: f32) {
        let uniforms = uniform! {
            max_iterations: max_iterations,
            complex_plane: complex_plane,
            pixel_size: pixel_size,
        };
    
        let mut target = self.display.draw();
        target.draw(
            &self.vertex_buffer,
            &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
            &self.program,
            &uniforms,
            &Default::default()
        ).unwrap();
    
        target.finish().unwrap();
    }
}

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_inner_size(LogicalSize::new(SCREEN_WIDTH, SCREEN_HEIGHT));
    let cb = glutin::ContextBuilder::new().with_vsync(true);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    // Compile the shaders
    let program = glium::Program::from_source(
        &display,
        include_str!("shader.glslv"),
        include_str!("shader.glslf"),
        None).unwrap();
    
    // Render 2 triangles covering the whole screen
    let vertices = [
        Vertex { position: [-1.0, 1.0] },
        Vertex { position: [1.0, 1.0] },
        Vertex { position: [-1.0, -1.0] },
        
        Vertex { position: [-1.0, -1.0] },
        Vertex { position: [1.0, 1.0] },
        Vertex { position: [1.0, -1.0] },
    ];
    
    let vertex_buffer = glium::VertexBuffer::new(&display, &vertices).unwrap();

    let context = Context::new(display, program, vertex_buffer);

    let max_iterations: i32 = 1000;
    let mut complex_plane = ComplexPlane::default();
    
    // Render loop
    let dim = context.display.get_context().get_framebuffer_dimensions();
    let (mut fitted_plane, mut pixel_size) = complex_plane.fit_to_screen(dim.0, dim.1);
    
    let mut redraw_needed = false;

    event_loop.run(move |event, _, control_flow| {
        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                glutin::event::WindowEvent::KeyboardInput {
                    device_id, input: kin, is_synthetic 
                } => {  
                    if kin.state == glutin::event::ElementState::Pressed {
                        if kin.virtual_keycode == Some(winit::event::VirtualKeyCode::W) {
                            complex_plane = complex_plane.move_down(-10.0); redraw_needed = true;
                        }
                        else if kin.virtual_keycode == Some(winit::event::VirtualKeyCode::S) {
                            complex_plane = complex_plane.move_down(10.0); redraw_needed = true;
                        }
                        else if kin.virtual_keycode == Some(winit::event::VirtualKeyCode::A) {
                            complex_plane = complex_plane.move_left(10.0); redraw_needed = true;
                        }
                        else if kin.virtual_keycode == Some(winit::event::VirtualKeyCode::D) {
                            complex_plane = complex_plane.move_left(-10.0); redraw_needed = true;
                        }
                        else if kin.virtual_keycode == Some(winit::event::VirtualKeyCode::R) {
                            complex_plane = complex_plane.zoom(0.5); redraw_needed = true;
                        }
                        else if kin.virtual_keycode == Some(winit::event::VirtualKeyCode::F) {
                            complex_plane = complex_plane.zoom(2.0); redraw_needed = true;
                        }
                    }

                    if (redraw_needed) {
                        redraw_needed = false;
                        let a = complex_plane.fit_to_screen(dim.0, dim.1);
                        fitted_plane = a.0;
                        pixel_size = a.1;
                        context.redraw(max_iterations, fitted_plane, pixel_size);
                    }
                },
                _ => return,
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            glutin::event::Event::RedrawRequested(_) => {
                context.redraw(max_iterations, fitted_plane, pixel_size);
            }
            _ => return,
        }

        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
    });
}



/*
fn main() -> Result<(), Error> {
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let (window, p_width, p_height, mut _hidpi_factor) =
        create_window("Conway's Game of Life", &event_loop);

    let surface_texture = SurfaceTexture::new(p_width, p_height, &window);

    let mut scale: f32 = 300.0;
    let mut center: (f32, f32) = (00.0,0.0);
    let mut changed: bool = false;

    let mut pixels = Pixels::new(SCREEN_WIDTH, SCREEN_HEIGHT, surface_texture)?;
    let mut fractal = Fractal::new(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32, scale, center);

    event_loop.run(move |event, _, control_flow| {
        // The one and only event that winit_input_helper doesn't have for us...
        if let Event::RedrawRequested(_) = event {

            fractal.readraw(pixels.get_frame());

            // ===== util
            if pixels
                .render()
                .map_err(|e| error!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // For everything else, for let winit_input_helper collect events to build its state.
        // It returns `true` when it is time to update our game state and request a redraw.
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if input.key_pressed(VirtualKeyCode::W) { center.1 -= 10.0 / scale as f32; changed = true }
            if input.key_pressed(VirtualKeyCode::A) { center.0 -= 10.0 / scale as f32; changed = true }
            if input.key_pressed(VirtualKeyCode::S) { center.1 += 10.0 / scale as f32; changed = true }
            if input.key_pressed(VirtualKeyCode::D) { center.0 += 10.0 / scale as f32; changed = true }
            if input.key_pressed(VirtualKeyCode::R) { scale *= 2.0; changed = true }
            if input.key_pressed(VirtualKeyCode::F) { scale /= 2.0; changed = true }
            if (changed) {
                changed = false;
                fractal = Fractal::new(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32, scale, center);
                for chunk in pixels.get_frame().chunks_exact_mut(4) {
                    chunk.copy_from_slice(&[0,0,0,255]);
                }
            }
            // Adjust high DPI factor
            if let Some(factor) = input.scale_factor_changed() {
                _hidpi_factor = factor;
            }
            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
            }
            window.request_redraw();
        }
    });
}
*/

#[derive(Clone, PartialEq)]
enum CellState {
    OutOfSet, InSet, Unknown
}


#[derive(Clone)]
struct Cell {
    z: Complex<f32>,
    c: Complex<f32>,
    iter_count: usize,
    i: usize,
    state: CellState
}

impl Cell {
    fn new(c: Complex<f32>, associated_pixel_index: usize) -> Self {
        return Cell { z: Complex::default(), c, iter_count: 0, i: associated_pixel_index, 
            state: CellState::Unknown };
    }

    fn check_for_main_cordioid(&mut self) {
        let ro = ((self.c.re - 0.25).powi(2)+self.c.im.powi(2)).sqrt();
        let th = self.c.im.atan2(self.c.re - 0.25);
        let ro_c = 0.5 - 0.5 * th.cos();
        if ro < ro_c {
            self.state = CellState::InSet;
        }
    }

    fn update(&mut self, times: usize) {
        for i in 0..times {
            self.z = self.z * self.z + self.c;
            if self.z.norm_sqr() > 4.0 {
                self.iter_count += i + 1;
                self.state = CellState::OutOfSet;
                return;
            }
        }
        self.iter_count += times;
        self.state = CellState::Unknown;
    }
}

struct Fractal {
    grid: Vec<Cell>
}

impl Fractal {
    fn new(width: i32, height: i32, scale: f32, center: (f32, f32)) -> Self {
        let fractal = Fractal { grid: (0..(width*height))
            .map(|i| {
                let pixel_x = i as i32 % width;
                let pixel_y = i as i32 / width;
                let x = ((pixel_x - (width  / 2)) as f32 ) / scale + center.0;
                let y = ((pixel_y - (height / 2)) as f32 ) / scale + center.1;
                let z = Complex { re: x, im: y };
                let mut cell = Cell::new(z, i as usize);
                cell.check_for_main_cordioid();
                return cell;
            }).collect()};
        return fractal;
    }
    fn readraw(&mut self, screen: &mut [u8]) {
        let mut new_grid = Vec::with_capacity(self.grid.len());
        self.grid.par_iter_mut()
            .filter(|c| c.state != CellState::InSet)
            .for_each(|c| c.update(10));
        for each in self.grid.iter() {
            match each.state {
                CellState::OutOfSet => {
                    let color = HSL { 
                        h: 270.0 * (1.0 - iter_count_to_percent(each.iter_count)),
                        s: 1.0, 
                        l: 0.7 
                    };
                    let rgb: (u8, u8, u8) = color.to_rgb();
                    screen[each.i * 4 + 0] = rgb.0; 
                    screen[each.i * 4 + 1] = rgb.1; 
                    screen[each.i * 4 + 2] = rgb.2; 
                }

                CellState::InSet => {
                    screen[each.i * 4 + 0] = 0; 
                    screen[each.i * 4 + 1] = 0; 
                    screen[each.i * 4 + 2] = 0; 
                }
                CellState::Unknown => {
                    new_grid.push(each.clone());
                }
            }
        }

        self.grid = new_grid;
    }
}

fn iter_count_to_percent(iter_count: usize) -> f64{
    return 1.0 - (1.0 / ( iter_count as f64 / 10.0 + 1.0 ) );
}


// COPYPASTE: ideally this could be shared.

/// Create a window for the game.
///
/// Automatically scales the window to cover about 2/3 of the monitor height.
///
/// # Returns
///
/// Tuple of `(window, surface, width, height, hidpi_factor)`
/// `width` and `height` are in `PhysicalSize` units.
fn create_window(
    title: &str,
    event_loop: &EventLoop<()>,
) -> (winit::window::Window, u32, u32, f64) {
    // Create a hidden window so we can estimate a good default window size
    let window = winit::window::WindowBuilder::new()
        .with_visible(false)
        .with_title(title)
        .build(event_loop)
        .unwrap();
    let hidpi_factor = window.scale_factor();

    // Get dimensions
    let width = SCREEN_WIDTH as f64;
    let height = SCREEN_HEIGHT as f64;
    let (monitor_width, monitor_height) = {
        if let Some(monitor) = window.current_monitor() {
            let size = monitor.size().to_logical(hidpi_factor);
            (size.width, size.height)
        } else {
            (width, height)
        }
    };
    let scale = (monitor_height / height * 2.0 / 3.0).round().max(1.0);

    // Resize, center, and display the window
    let min_size: winit::dpi::LogicalSize<f64> =
        PhysicalSize::new(width, height).to_logical(hidpi_factor);
    let default_size = LogicalSize::new(width * scale, height * scale);
    let center = LogicalPosition::new(
        (monitor_width - width * scale) / 2.0,
        (monitor_height - height * scale) / 2.0,
    );
    window.set_inner_size(default_size);
    window.set_min_inner_size(Some(min_size));
    window.set_outer_position(center);
    window.set_visible(true);

    let size = default_size.to_physical::<f64>(hidpi_factor);

    (
        window,
        size.width.round() as u32,
        size.height.round() as u32,
        hidpi_factor,
    )
}
