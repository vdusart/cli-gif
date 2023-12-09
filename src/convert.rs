pub struct Frame {
    pub width: u16,
    pub height: u16,
    pub pixels: Vec<(u8, u8, u8)>,
    pub grayscale_chars: Vec<char>,
}

impl Frame {
    pub fn new(w: u16, h: u16) -> Self {
        Frame {
            width: w,
            height: h,
            pixels: Vec::new(),
            grayscale_chars: Vec::new(),
        }
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
