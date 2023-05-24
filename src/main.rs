#![allow(unused)]
use easer::functions::*;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

use noize::{Ease, File, PNoise1};

fn main() {
    //textures().unwrap();
    image();
    //sound();
}

fn sound() {
    let noise = PNoise1::new(47, 4096, 44100, Ease::SmoothStep);
    File::Wav("out.wav", noise.output.as_slice()).save();
    File::Png("out_wav.png", noise.output.as_slice()).save();
}

fn image() {
    //let noise = PNoise1::new(47, 64, 1024, Ease::Bounce);
    let noise = PNoise1::new(47, 16, 1024, Ease::SmoothStep);
    File::Png("out_noise.png", noise.output.as_slice()).save();
    File::Png("out_grid.png", noise.grid.as_slice()).save();
}
