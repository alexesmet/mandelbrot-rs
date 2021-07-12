use glium::uniforms::{UniformValue, AsUniformValue};
use num_complex::Complex;

/// Representation of the coordinate space of the mandelbrot set.
/// Has upper and lower real and imaginary boundaries.
#[derive(Clone, Debug, Copy)]
pub struct ComplexPlane {
    min: Complex<f32>,
    max: Complex<f32>,
    // image size in pixel
    width: u32,
    height: u32,
}

impl ComplexPlane {
    
    // --- Math ---
    
    /// Computes how big one pixel is in the complex plane.
    /// Adjust the aspect ratio to 1:1.
    fn pixel_size(&self) -> f32 {
        let pixel_width = (self.max.re - self.min.re) / self.width as f32;
        let pixel_height = (self.max.im - self.min.im) / self.height as f32;
        f32::max(pixel_width, pixel_height)
    }
    
    // --- Events ---
    
    /// Resize the plane to fit into the window without
    /// stretching it in one direction or the other.
    pub fn fit_to_screen(&self, width: u32, height: u32) -> (ComplexPlane, f32) {
        let resized = ComplexPlane {
            height,
            width,
            .. self.clone() // rest is the same.
        };
        // math is the same as noop-zoom. pixel_size() does the magic.
        let new_plane = resized.zoom(1.0);
        let pixel_size = new_plane.pixel_size();
        (new_plane, pixel_size)
    }

    /// Negative pixels move it to right.
    pub fn move_left(&self, pixels: f32) -> ComplexPlane {
        ComplexPlane {
            min: Complex::<f32> {re: (self.min.re - pixels * self.pixel_size()), im: self.min.im},
            max: Complex::<f32> {re: (self.max.re - pixels * self.pixel_size()), im: self.max.im},
            .. self.clone()
        }
    }
    
    /// Negative pixels move it upwards.
    pub fn move_down(&self, pixels: f32) -> ComplexPlane {
        ComplexPlane {
            min: Complex::<f32> {re: self.min.re, im: (self.min.im - pixels * self.pixel_size())},
            max: Complex::<f32> {re: self.max.re, im: (self.max.im - pixels * self.pixel_size())},
            .. self.clone()
        }
    }
    
    /// Zooms in (`factor < 1.0`) or out (`factor > 1.0`).
    /// The center point remains stationary.
    pub fn zoom(&self, factor: f32) -> ComplexPlane {
        let new_pixel_size = factor * self.pixel_size();
        
        let center = (self.min + self.max) / 2.0;
        let radius = Complex::<f32> {
            re : self.width as f32 * new_pixel_size / 2.0, 
            im: self.height as f32 * new_pixel_size / 2.0
        };
    
        ComplexPlane {
            min: center - radius,
            max: center + radius,
            width: self.width,
            height: self.height,
        }
    }
}

impl Default for ComplexPlane {
    fn default() -> Self {
        ComplexPlane {
            min: Complex::<f32> {re: -2.0, im: -1.2},
            max: Complex::<f32> {re: 0.8, im: 1.2},
            width: 1000,
            height: 1000,
        }
    }
}

/// So glium can convert it to a vec4
impl AsUniformValue for ComplexPlane {
    fn as_uniform_value(&self) -> UniformValue {
        UniformValue::Vec4([self.min.re, self.min.im, self.max.re, self.max.im])
    }
}