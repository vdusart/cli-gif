use crate::convert::GifFrame;

impl GifFrame {
    pub fn display(&self) {
        // println!("Frame: w: {}, h: {}, delay: {}", self.width, self.height, self.delay);

        // println!("RGB:");
        // for (i, p) in self.pixels.iter().enumerate() {
        //     if (i as u64) % self.width == 0 {
        //         println!();
        //     }
        //     print!("{:?}", p);
        // }
        // println!();
        
        // println!("Grayscale:");
        print!("{}[2J", 27 as char);    // Clear terminal

        for (i, c) in self.grayscale_chars.iter().enumerate() {
            if (i as u64) % self.width == 0 {
                println!();
            }
            print!("{c}");
        }
        println!();
    }
}
