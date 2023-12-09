fn get_gif_bytes_from_url(url: &str) -> Vec<u8> {
    let response = minreq::get(url).send().unwrap();
    assert_eq!(200, response.status_code, "Invalid Status Code");
    response.as_bytes().to_vec()
}

mod convert;
mod display;

fn main() {
    let gif_url: &str = "http://0.0.0.0:8000/default.gif";

    let gif_data: &[u8] = &get_gif_bytes_from_url(gif_url)[..];

    let mut decoder = gif::DecodeOptions::new();
    decoder.set_color_output(gif::ColorOutput::RGBA);

    let mut decoder = decoder.read_info(gif_data).unwrap();
    while let Some(frame) = decoder.read_next_frame().unwrap() {
        // println!("\n-- new frame --");
        // println!("width: {:?}px", frame.width);
        // println!("height: {:?}px", frame.height);
        // println!("buffer: {:?}", frame.buffer);
        // println!("nb of pixel: {}", frame.buffer.len() / 4);
        let mut test = convert::Frame::new(frame.width, frame.height);

        test.add_pixels(frame.buffer.to_vec());
        test.create_grayscale();
        test.display();
    }
}
