use gif::Frame;

pub struct GifFrame {
    pub width: u64,
    pub height: u64,
    pub delay: u64,
    pub pixels: Vec<(u8, u8, u8)>,
    pub grayscale_chars: Vec<char>,
}

impl GifFrame {
    pub fn new(f: &Frame) -> Self {
        let mut frame = GifFrame {
            width: f.width as u64,
            height: f.height as u64,
            delay: 250,
            pixels: Vec::new(),
            grayscale_chars: Vec::new(),
        };
        if f.delay > 0 {
            frame.delay = (f.delay as u64) * 10;
        }
        frame.add_pixels(f.buffer.to_vec());
        frame.create_grayscale();
        frame
    }

    pub fn add_pixels(&mut self, buffer: Vec<u8>) {
        for i in (0..self.width * self.height * 4).step_by(4) {
            let r = *buffer.get((i + 0) as usize).unwrap();
            let g = *buffer.get((i + 1) as usize).unwrap();
            let b = *buffer.get((i + 2) as usize).unwrap();
            self.pixels.push((r, g, b));
        }
    }

    // Todo: create a downgrade function that will reduce the frame image to a defined [COL, ROW] shape
    // pub fn downgrade(&mut self) {}

    pub fn create_grayscale(&mut self) {
        let gscale = "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/|()1{}[]?-_+~<>i!lI;:,\"^`'.";
        let n = gscale.len() as u16 - 1;
        for (r, g, b) in self.pixels.iter() {
            let grayscale = (0.299 * (*r as f64) + 0.587 * (*g as f64) + 0.114 * (*b as f64)) as u16;
            let gscale_index = grayscale * n / 255;
            let ascii_char = gscale.chars().nth(gscale_index as usize).unwrap_or(' ');
            self.grayscale_chars.push(ascii_char);
        }
    }
}
