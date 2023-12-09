use crate::convert::Frame;

impl Frame {
    pub fn display(&self) {
        // println!("Frame: w: {}, h: {}", self.width, self.height);

        // println!("RGB:");
        // for (i, p) in self.pixels.iter().enumerate() {
        //     if (i as u16) % self.width == 0 {
        //         println!();
        //     }
        //     print!("{:?}", p);
        // }
        // println!();
        
        // println!("Grayscale:");
        for (i, c) in self.grayscale_chars.iter().enumerate() {
            if (i as u16) % self.width == 0 {
                println!();
            }
            print!("{c}");
        }
        println!();
    }
}
