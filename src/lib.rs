#[allow(unused)]
use easer::functions::*;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

mod kernel;
mod texture;

pub use kernel::{NoiseKernel, NoiseKernelV1};
pub use texture::{make_noise, make_noise2, noise0, noise1, noise2, noise3, textures};

pub fn smooth_step(left: f32, right: f32, x: f32) -> f32 {
    left + (right - left) * x * x * (3.0 - 2.0 * x)
}

pub fn smoother_step(left: f32, right: f32, x: f32) -> f32 {
    left + (right - left) * x * x * x * (x * (x * 6.0 - 15.0) + 10.0)
}

pub fn smoothest_step(left: f32, right: f32, x: f32) -> f32 {
    left + (right - left) * x * x * x * x * (x * (x * (x * -20.0 + 70.0) - 84.0) + 35.0)
}

pub enum File<'a> {
    Png(&'a str, &'a [f32]),
    Wav(&'a str, &'a [f32]),
}

impl File<'_> {
    pub fn save(&self) {
        match self {
            File::Png(filename, bytes) => {
                let mut img = image::ImageBuffer::from_fn(bytes.len() as u32, 256, |x, y| {
                    let y_norm = y as f32 / 255.0;
                    if y_norm <= (bytes[x as usize] + 1.0) / 2.0 {
                        image::Rgba::<u8>::from([125, 0, 0, 255])
                    } else {
                        image::Rgba::<u8>::from([0, 0, 125, 255])
                    }
                });
                image::imageops::flip_vertical_in_place(&mut img);
                img.save(filename).unwrap();
            }
            File::Wav(filename, bytes) => {
                let spec = hound::WavSpec {
                    channels: 1,
                    sample_rate: 44100,
                    bits_per_sample: 32,
                    sample_format: hound::SampleFormat::Float,
                };
                let mut writer = hound::WavWriter::create(filename, spec).unwrap();
                for s in *bytes {
                    writer.write_sample(*s).unwrap();
                }
                writer.finalize().unwrap();
            }
        }
    }
}

pub enum Ease {
    SmoothStep,
    SmootherStep,
    SmoothestStep,
    Back,
    Bounce,
    Circ,
    Cubic,
    Elastic,
    Expo,
    Linear,
    Quad,
    Quart,
    Quint,
    Sine,
}

pub struct PNoise1 {
    pub grid: Vec<f32>,
    pub grid_len: usize,
    pub output: Vec<f32>,
    pub output_len: usize,
    pub seed: u64,
    pub rng: ChaCha8Rng,
    x: usize,
}

impl PNoise1 {
    pub fn new(seed: u64, grid_len: usize, output_len: usize, ease: Ease) -> Self {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        let mut grid = Vec::with_capacity(grid_len);

        for _ in 0..grid_len {
            grid.push(rng.gen_range(-1.0..=1.0));
        }

        let mut output = Vec::with_capacity(output_len);

        for x in 0..output_len {
            let grid_x = x as f32 * grid_len as f32 / output_len as f32;
            let node_left = grid_x.floor() as usize;
            let node_right = if node_left == grid_len - 1 {
                0
            } else {
                grid_x.floor() as usize + 1
            };
            let dx = grid_x - node_left as f32;

            let interpolated = match ease {
                Ease::SmoothStep => smooth_step(grid[node_left], grid[node_right], dx),
                Ease::SmootherStep => smoother_step(grid[node_left], grid[node_right], dx),
                Ease::SmoothestStep => smoothest_step(grid[node_left], grid[node_right], dx),
                Ease::Back => Back::ease_in_out(dx, grid[node_left], grid[node_right], 1.0),
                Ease::Bounce => Bounce::ease_in_out(dx, grid[node_left], grid[node_right], 1.0),
                Ease::Circ => Circ::ease_in_out(dx, grid[node_left], grid[node_right], 1.0),
                Ease::Cubic => Cubic::ease_in_out(dx, grid[node_left], grid[node_right], 1.0),
                Ease::Elastic => Elastic::ease_in_out(dx, grid[node_left], grid[node_right], 1.0),
                Ease::Expo => Expo::ease_in_out(dx, grid[node_left], grid[node_right], 1.0),
                Ease::Linear => Linear::ease_in_out(dx, grid[node_left], grid[node_right], 1.0),
                Ease::Quad => Quad::ease_in_out(dx, grid[node_left], grid[node_right], 1.0),
                Ease::Quart => Quart::ease_in_out(dx, grid[node_left], grid[node_right], 1.0),
                Ease::Quint => Quint::ease_in_out(dx, grid[node_left], grid[node_right], 1.0),
                Ease::Sine => Sine::ease_in_out(dx, grid[node_left], grid[node_right], 1.0),
            };

            output.push(interpolated);
        }

        Self {
            seed,
            grid,
            grid_len,
            output,
            output_len,
            rng,
            x: 0,
        }
    }
}

impl Iterator for PNoise1 {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        let result = self.output[self.x];

        if self.x == self.output_len - 1 {
            self.x = 0;
        } else {
            self.x += 1;
        }

        Some(result)
    }
}

pub struct Noise1V0 {
    pub grid: Vec<f32>,
    pub grid_len: usize,
    pub output: Vec<f32>,
    pub output_len: usize,
    pub seed: u64,
    pub rng: ChaCha8Rng,
}

impl Noise1V0 {
    pub fn new(seed: u64, grid_len: usize, output_len: usize) -> Self {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        let mut grid = Vec::with_capacity(grid_len);

        for _ in 0..grid_len {
            grid.push(rng.gen_range(-1.0..=1.0));
        }

        let mut output = Vec::with_capacity(output_len);

        for x in 0..output_len {
            let grid_x = x as f32 * grid_len as f32 / output_len as f32;
            let node_left = grid_x.floor() as usize;
            let node_right = if node_left == grid_len - 1 {
                0
            } else {
                grid_x.floor() as usize + 1
            };
            let dx = grid_x - node_left as f32;
            let dot_left = grid[node_left] * dx;

            //let dot_right = grid[node_right] * (dx - 1.0);
            let dot_right = grid[node_right] * (1.0 - dx);

            let interpolated = dot_left * dx + dot_right * (1.0 - dx);

            output.push(interpolated);
        }

        Self {
            seed,
            grid,
            grid_len,
            output,
            output_len,
            rng,
        }
    }
}
