use crate::convert::GifFrame;

impl GifFrame {
    pub fn display(&self) {
        // Clear terminal
        print!("{}[2J", 27 as char);

        for (i, c) in self.grayscale_chars.iter().enumerate() {
            if i % self.resized_width == 0 {
                println!();
            }
            print!("{c}");
        }
        println!();
    }
}
