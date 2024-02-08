mod convert;
mod display;
mod types;

use convert::GifFrame;
use std::thread::sleep;
use std::time::Duration;
use types::DisplayOptions;

fn get_gif_bytes_from_url(url: &str) -> Vec<u8> {
    let response = minreq::get(url).send().unwrap();
    assert_eq!(200, response.status_code, "Invalid Status Code");
    response.as_bytes().to_vec()
}

fn main() {
    let gif_url: &str = "https://media2.giphy.com/media/v1.Y2lkPTc5MGI3NjExcm5uNjFkZjhtZ3l4aTB2NjhnMXVubG84MWc4bzBtY3BoajdoY3AwdCZlcD12MV9pbnRlcm5hbF9naWZfYnlfaWQmY3Q9Zw/GlWeBb6U3R1PpjH9VV/giphy.gif";

    let gif_data: &[u8] = &get_gif_bytes_from_url(gif_url)[..];

    let mut decoder = gif::DecodeOptions::new();
    decoder.set_color_output(gif::ColorOutput::RGBA);

    let mut frames: Vec<GifFrame> = Vec::new();

    let mut decoder = decoder.read_info(gif_data).unwrap();
    let options = DisplayOptions::default();
    while let Some(frame) = decoder.read_next_frame().unwrap() {
        frames.push(convert::GifFrame::new(frame, &options));
    }

    for frame in frames.iter().cycle() {
        sleep(Duration::from_millis(frame.delay));
        frame.display();
    }
}
