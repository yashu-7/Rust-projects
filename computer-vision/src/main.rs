use opencv::prelude::*;
use opencv::videoio;
use opencv::highgui;

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
        if frame.size()?.width == 0 {
            break;
        }
        highgui::imshow("Frame", &frame)?;
        if highgui::wait_key(10)? == 'q' as i32 {
            break;
        }
    }
    Ok(())
}