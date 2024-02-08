use gif::Frame;
use std::cmp::min;

use crate::types::{Color, DisplayOptions};

pub struct GifFrame {
    pub width: usize,
    pub height: usize,
    pub resized_width: usize,
    pub resized_height: usize,
    pub delay: u64,
    pub pixels: Vec<Color>,
    pub grayscale_chars: Vec<char>,
}

impl GifFrame {
    pub fn new(f: &Frame, options: &DisplayOptions) -> Self {
        let mut frame = GifFrame {
            width: f.width as usize,
            height: f.height as usize,
            resized_width: 0,
            resized_height: 0,
            delay: f.delay as u64 * 10,
            pixels: Vec::new(),
            grayscale_chars: Vec::new(),
        };
        if f.delay <= 0 {
            frame.delay = 250;
        }
        frame.add_pixels(f.buffer.to_vec());
        frame.downsize(options);
        frame.create_grayscale();
        frame
    }

    pub fn add_pixels(&mut self, buffer: Vec<u8>) {
        for i in (0..self.width * self.height * 4).step_by(4) {
            let r = *buffer.get((i + 0) as usize).unwrap();
            let g = *buffer.get((i + 1) as usize).unwrap();
            let b = *buffer.get((i + 2) as usize).unwrap();
            self.pixels.push(Color::new(r, g, b));
        }
    }

    fn extract_row(&self, arr: &Vec<Color>, width: usize, index: usize) -> Vec<Color> {
        let start = index * width;
        let end = start + width;
        arr[start..end].to_vec()
    }

    fn extract_col(&self, arr: &Vec<Color>, width: usize, index: usize) -> Vec<Color> {
        let mut col: Vec<Color> = Vec::new();
        for j in 0..(arr.len() / width) {
            col.push(*arr.get(index + j * width).unwrap());
        }
        col
    }

    fn get_mean(&self, arr: &[Color]) -> Color {
        let mut mean: Color = Color::new(0, 0, 0);
        let n = arr.len() as u8;
        for color in arr.iter() {
            mean = mean + *color / n;
        }
        mean
    }

    pub fn downsize(&mut self, options: &DisplayOptions) {
        let ow = self.width;
        let oh = self.height;

        let nw: usize = min(ow, options.width);
        let nh: usize = min(oh, options.height);
        self.resized_width = nw;
        self.resized_height = nh;

        // Horizontal resize
        let mut h_resized: Vec<Color> = Vec::new();
        for h in 0..oh {
            let r = self.extract_row(&self.pixels, ow, h);
            let k = ow / nw;
            let m = ow % nw;

            for i in 0..nw {
                let start = i * k + min(i, m);
                let end = (i + 1) * k + min(i + 1, m);
                h_resized.push(self.get_mean(&r[start..end]));
            }
        }

        // Vertical resize
        let mut v_resized: Vec<Color> = Vec::new();
        for h in 0..nw {
            let c = self.extract_col(&h_resized, nw, h);
            let k = oh / nh;
            let m = oh % nh;
            for i in 0..nh {
                let start = i * k + min(i, m);
                let end = (i + 1) * k + min(i + 1, m);
                v_resized.push(self.get_mean(&c[start..end]));
            }
        }

        // Rearrange pixels array
        let mut resized: Vec<Color> = Vec::new();
        for i in 0..nh {
            for j in 0..nw {
                resized.push(*v_resized.get(i + j * nh).unwrap());
            }
        }

        self.pixels = resized;
    }

    pub fn create_grayscale(&mut self) {
        let gscale = "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,\"^`'. ";
        let n = gscale.len() as u16 - 1;
        for color in self.pixels.iter() {
            let grayscale = (0.299 * (color.r as f64)
                + 0.587 * (color.g as f64)
                + 0.114 * (color.b as f64)) as u16;
            let gscale_index = grayscale * n / 255;
            let ascii_char = gscale.chars().nth(gscale_index as usize).unwrap_or(' ');
            self.grayscale_chars.push(ascii_char);
        }
    }
}
