use opencv::core::Size;
use opencv::highgui;
use opencv::prelude::*;
use opencv::videoio;

mod filters;

fn main() -> opencv::Result<()> {
    let height: f64 = 480.0;
    let width: f64 = 640.0;
    let fps: f64 = 30.0;
    let mut cap = videoio::VideoCapture::new(0, videoio::CAP_DSHOW)?;
    let _ = cap.set(videoio::CAP_PROP_XI_HEIGHT, height);
    let _ = cap.set(videoio::CAP_PROP_XI_WIDTH, width);
    let _ = cap.set(videoio::CAP_PROP_FPS, fps);
    if !cap.is_opened()? {
        panic!("Error opening video stream or file");
    }
    loop {
        let mut frame = Mat::default();
        cap.read(&mut frame)?;
        let flipped_frame = {
            let mut flipped = Mat::default();
            opencv::core::flip(&frame, &mut flipped, 1)?;
            flipped
        };
        let ksize: i32 = 13;
        let k_size = Size::new(3,3);
        let sigmax: f64 = 5.5;
        let sigmay: f64 = 5.0;
        let border_type: i32 = 3;
        let median_blur = filters::filters::median_blur(&flipped_frame, ksize)?;
        let guassian_blur = filters::filters::guassian_blur(&flipped_frame, k_size,sigmax,sigmay,border_type)?;
        if flipped_frame.size()?.width == 0 {
            break;
        }
        highgui::imshow("Frame", &flipped_frame)?;
        highgui::imshow("Median Blur", &median_blur)?;
        highgui::imshow("Guassian Blur", &guassian_blur)?;
        if highgui::wait_key(1)? == 'q' as i32 {
            break;
        }
    }
    Ok(())
}