pub use cgmath::{dot, prelude::*, vec2, vec4, Vector2, Vector4};
use image::{Rgba, RgbaImage};
use ndarray::{Array2, Dim};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

pub struct NoiseKernel {
    pub out_width: usize,
    pub out_height: usize,
    pub scale_x: usize,
    pub scale_y: usize,
    pub seed: u64,
}

impl Default for NoiseKernel {
    fn default() -> Self {
        Self {
            out_width: 1024,
            out_height: 1024,
            scale_x: 64,
            scale_y: 64,
            seed: 47,
        }
    }
}

impl NoiseKernel {
    pub fn make_noise<F>(&self, f: F) -> Array2<Vector4<f32>>
    where
        F: Fn(Vector2<f32>, Vector2<f32>, Vector2<f32>, Vector2<f32>, f32, f32) -> Vector4<f32>,
    {
        let mut rng = ChaCha8Rng::seed_from_u64(self.seed);
        let mut grid = Array2::<Vector2<f32>>::zeros(Dim([self.scale_x, self.scale_y]));
        let mut noise = Array2::<Vector4<f32>>::zeros(Dim([self.out_width, self.out_height]));

        grid.map_inplace(|elem| {
            elem.x = rng.gen_range(-1.0..=1.0);
            elem.y = rng.gen_range(-1.0..=1.0);
            elem.normalize();
        });

        for (i, elem) in noise.iter_mut().enumerate() {
            let x = i % self.out_width;
            let y = i / self.out_width;

            let map_x = x as f32 * self.scale_x as f32 / self.out_width as f32;
            let map_y = y as f32 * self.scale_y as f32 / self.out_height as f32;

            let floor_x = map_x.floor() as u32;
            let floor_y = map_y.floor() as u32;

            let wrapped_x = if (floor_x + 1) as usize == self.scale_x {
                0
            } else {
                floor_x + 1
            };
            let wrapped_y = if (floor_y + 1) as usize == self.scale_y {
                0
            } else {
                floor_y + 1
            };

            let tl = grid[[floor_x as usize, floor_y as usize]];
            let bl = grid[[floor_x as usize, wrapped_y as usize]];
            let tr = grid[[wrapped_x as usize, floor_y as usize]];
            let br = grid[[wrapped_x as usize, wrapped_y as usize]];

            *elem = f(
                tl,
                bl,
                tr,
                br,
                map_x - floor_x as f32,
                map_y - floor_y as f32,
            );
        }

        noise
    }
}

pub struct NoiseKernelV1 {
    pub out_width: u32,
    pub out_height: u32,
    pub scale_x: u32,
    pub scale_y: u32,
    pub seed: u64,
}

impl Default for NoiseKernelV1 {
    fn default() -> Self {
        Self {
            out_width: 1024,
            out_height: 1024,
            scale_x: 64,
            scale_y: 64,
            seed: 47,
        }
    }
}

impl NoiseKernelV1 {
    pub fn make_noise<F>(&self, f: F) -> RgbaImage
    where
        F: Fn(&Rgba<u8>, &Rgba<u8>, &Rgba<u8>, &Rgba<u8>, f32, f32, f32, f32) -> Rgba<u8>,
    {
        let mut grid = RgbaImage::new(self.scale_x, self.scale_y);
        let mut noise = RgbaImage::new(self.out_width, self.out_height);
        let mut rng = ChaCha8Rng::seed_from_u64(self.seed);

        for (_x, _y, vector) in grid.enumerate_pixels_mut() {
            *vector = Rgba::from([rng.gen(), rng.gen(), rng.gen(), 255]);
        }

        for (x, y, color) in noise.enumerate_pixels_mut() {
            let map_x = x as f32 * self.scale_x as f32 / self.out_width as f32;
            let map_y = y as f32 * self.scale_y as f32 / self.out_height as f32;
            let floor_x = map_x.floor() as u32;
            let floor_y = map_y.floor() as u32;
            let wrapped_x = if floor_x + 1 == self.scale_x {
                0
            } else {
                floor_x + 1
            };
            let wrapped_y = if floor_y + 1 == self.scale_y {
                0
            } else {
                floor_y + 1
            };
            let tl = grid.get_pixel(floor_x, floor_y);
            let bl = grid.get_pixel(floor_x, wrapped_y);
            let tr = grid.get_pixel(wrapped_x, floor_y);
            let br = grid.get_pixel(wrapped_x, wrapped_y);

            let d_tl = 1.0
                - ((map_x - map_x.floor()).powi(2) + (map_y - map_y.floor()).powi(2)).sqrt()
                    / 2f32.sqrt();
            let d_bl = 1.0
                - ((map_x - map_x.floor()).powi(2) + (map_y.floor() + 1.0 - map_y).powi(2)).sqrt()
                    / 2f32.sqrt();
            let d_tr = 1.0
                - ((map_x.floor() + 1.0 - map_x).powi(2) + (map_y - map_y.floor()).powi(2)).sqrt()
                    / 2f32.sqrt();
            let d_br = 1.0
                - ((map_x.floor() + 1.0 - map_x).powi(2) + (map_y.floor() + 1.0 - map_y).powi(2))
                    .sqrt()
                    / 2f32.sqrt();

            *color = f(tl, bl, tr, br, d_tl, d_bl, d_tr, d_br);
        }

        noise
    }
}
