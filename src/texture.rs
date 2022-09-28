use crate::*;

use cgmath::{dot, vec2, vec4};
use image::{Rgba, RgbaImage};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

pub fn make_noise(width: u32, height: u32, scale_x: u32, scale_y: u32, seed: u64) -> RgbaImage {
    let mut grid = RgbaImage::new(scale_x, scale_y);
    let mut noise = RgbaImage::new(width, height);

    let mut rng = ChaCha8Rng::seed_from_u64(seed);

    for (_x, _y, vector) in grid.enumerate_pixels_mut() {
        *vector = Rgba::from([rng.gen(), rng.gen(), rng.gen(), 0]);
    }

    for (x, y, color) in noise.enumerate_pixels_mut() {
        let map_x = x as f32 * scale_x as f32 / width as f32;
        let map_y = y as f32 * scale_y as f32 / height as f32;
        let floor_x = map_x.floor() as u32;
        let floor_y = map_y.floor() as u32;
        let wrapped_x = if floor_x + 1 == scale_x {
            0
        } else {
            floor_x + 1
        };
        let wrapped_y = if floor_y + 1 == scale_y {
            0
        } else {
            floor_y + 1
        };
        let tl = grid.get_pixel(floor_x, floor_y);
        let bl = grid.get_pixel(floor_x, wrapped_y);
        let tr = grid.get_pixel(wrapped_x, floor_y);
        let br = grid.get_pixel(wrapped_x, wrapped_y);

        let d_tl = ((map_x - map_x.floor()).powi(2) + (map_y - map_y.floor()).powi(2)).sqrt()
            / 2f32.sqrt();
        let d_bl = ((map_x - map_x.floor()).powi(2) + (map_y.floor() + 1.0 - map_y).powi(2)).sqrt()
            / 2f32.sqrt();
        let d_tr = ((map_x.floor() + 1.0 - map_x).powi(2) + (map_y - map_y.floor()).powi(2)).sqrt()
            / 2f32.sqrt();
        let d_br = ((map_x.floor() + 1.0 - map_x).powi(2) + (map_y.floor() + 1.0 - map_y).powi(2))
            .sqrt()
            / 2f32.sqrt();

        let r = (tl.0[0] as f32 * d_tl
            + bl.0[0] as f32 * d_bl
            + tr.0[0] as f32 * d_tr
            + br.0[0] as f32 * d_br)
            / 4.0;
        let g = (tl.0[1] as f32 * d_tl
            + bl.0[1] as f32 * d_bl
            + tr.0[1] as f32 * d_tr
            + br.0[1] as f32 * d_br)
            / 4.0;
        let b = (tl.0[2] as f32 * d_tl
            + bl.0[2] as f32 * d_bl
            + tr.0[2] as f32 * d_tr
            + br.0[2] as f32 * d_br)
            / 4.0;

        *color = Rgba::from([r.floor() as u8, g.floor() as u8, b.floor() as u8, 255]);
    }

    noise
}

pub fn make_noise2(width: u32, height: u32, scale_x: u32, scale_y: u32, seed: u64) -> RgbaImage {
    let mut grid = RgbaImage::new(scale_x, scale_y);
    let mut noise = RgbaImage::new(width, height);

    let mut rng = ChaCha8Rng::seed_from_u64(seed);

    for (_x, _y, vector) in grid.enumerate_pixels_mut() {
        *vector = Rgba::from([rng.gen(), rng.gen(), rng.gen(), 0]);
    }

    for (x, y, color) in noise.enumerate_pixels_mut() {
        let map_x = x as f32 * scale_x as f32 / width as f32;
        let map_y = y as f32 * scale_y as f32 / height as f32;
        let floor_x = map_x.floor() as u32;
        let floor_y = map_y.floor() as u32;
        let wrapped_x = if floor_x + 1 == scale_x {
            0
        } else {
            floor_x + 1
        };
        let wrapped_y = if floor_y + 1 == scale_y {
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

        let r = (tl.0[0] as f32 * d_tl
            + bl.0[0] as f32 * d_bl
            + tr.0[0] as f32 * d_tr
            + br.0[0] as f32 * d_br)
            / 4.0;
        let g = (tl.0[1] as f32 * d_tl
            + bl.0[1] as f32 * d_bl
            + tr.0[1] as f32 * d_tr
            + br.0[1] as f32 * d_br)
            / 4.0;
        let b = (tl.0[2] as f32 * d_tl
            + bl.0[2] as f32 * d_bl
            + tr.0[2] as f32 * d_tr
            + br.0[2] as f32 * d_br)
            / 4.0;

        *color = Rgba::from([r.floor() as u8, g.floor() as u8, b.floor() as u8, 255]);
    }

    noise
}

pub fn noise0() -> RgbaImage {
    let noise = NoiseKernel {
        out_width: 1400,
        out_height: 1400,
        scale_x: 25,
        scale_y: 35,
        seed: 47,
    }
    .make_noise(|tl, bl, tr, br, x, y| {
        let r = dot(tl, vec2(x, y));
        let g = dot(bl, vec2(x, y));
        let b = dot(tr, vec2(x, y));
        let a = dot(br, vec2(x, y));

        vec4(r, g, b, a)
    });

    RgbaImage::from_fn(1400, 1400, |x, y| {
        let v = noise[[x as usize, y as usize]];
        Rgba::from([
            (v[0] * 255.) as u8,
            (v[1] * 255.) as u8,
            (v[2] * 255.) as u8,
            (v[3] * 255.) as u8,
        ])
    })
}

pub fn noise1() -> RgbaImage {
    NoiseKernelV1::default().make_noise(|tl, bl, tr, br, d_tl, d_bl, d_tr, d_br| {
        // Black grid with blurry color splotches.
        let r = (tl.0[0] as f32 * d_tl
            + bl.0[0] as f32 * d_bl
            + tr.0[0] as f32 * d_tr
            + br.0[0] as f32 * d_br)
            / 4.0;
        let g = (tl.0[1] as f32 * d_tl
            + bl.0[1] as f32 * d_bl
            + tr.0[1] as f32 * d_tr
            + br.0[1] as f32 * d_br)
            / 4.0;
        let b = (tl.0[2] as f32 * d_tl
            + bl.0[2] as f32 * d_bl
            + tr.0[2] as f32 * d_tr
            + br.0[2] as f32 * d_br)
            / 4.0;
        Rgba::from([r.floor() as u8, g.floor() as u8, b.floor() as u8, 255])
    })
}

pub fn noise2() -> RgbaImage {
    NoiseKernelV1 {
        scale_x: 26,
        scale_y: 26,
        ..NoiseKernelV1::default()
    }
    .make_noise(|tl, _bl, tr, br, d_tl, _d_bl, d_tr, d_br| {
        // Dark with triangular red and green gradient boxes.
        let r = tl.0[0] as f32 * d_tl;
        let g = br.0[0] as f32 * d_br;
        let b = tr.0[0] as f32 * d_tr;
        let a = br.0[0] as f32;
        Rgba::from([
            r.floor() as u8,
            g.floor() as u8,
            b.floor() as u8,
            a.floor() as u8,
        ])
    })
}

pub fn noise3() -> RgbaImage {
    NoiseKernelV1 {
        out_width: 1400,
        out_height: 1400,
        scale_x: 100,
        scale_y: 100,
        ..NoiseKernelV1::default()
    }
    .make_noise(|tl, _bl, tr, br, d_tl, _d_bl, d_tr, d_br| {
        let r = tl.0[0] as f32 * d_tl;
        let g = br.0[0] as f32 * d_br;
        let b = tr.0[0] as f32 * d_tr;
        let a = br.0[0] as f32;
        Rgba::from([
            r.floor() as u8,
            g.floor() as u8,
            b.floor() as u8,
            a.floor() as u8,
        ])
    })
}

pub fn textures() -> image::ImageResult<()> {
    noise0().save("noise0.png")?;
    noise1().save("noise1.png")?;
    noise2().save("noise2.png")?;
    noise3().save("noise3.png")?;
    make_noise(640, 480, 13, 13, 47).save("mknoise0.png")?;
    make_noise(640, 480, 130, 130, 47).save("mknoise1.png")?;
    make_noise(1024, 1024, 42, 42, 47).save("mknoise2.png")?;
    make_noise(1024, 1024, 30, 80, 47).save("mknoise3.png")?;
    make_noise(1024, 1024, 60, 50, 47).save("mknoise4.png")?;
    make_noise2(1024, 1024, 60, 50, 47).save("mknoise5.png")?;
    make_noise2(1024, 1024, 6, 5, 47).save("mknoise6.png")
}
