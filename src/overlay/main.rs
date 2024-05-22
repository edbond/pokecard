use std::time::{Duration, Instant};

use opencv::core::{Mat, MatTraitConst, CV_32FC4, CV_32SC4, CV_8UC4};
use opencv::imgproc::{cvt_color, rectangle_def, COLOR_BGR2BGRA, LINE_8};
use opencv::prelude::*;
use opencv::{
    core::{Rect, Scalar},
    highgui, videoio, Result,
};

fn main() -> Result<()> {
    let window = "video capture";

    highgui::named_window(window, highgui::WINDOW_AUTOSIZE)?;

    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?; // 0 is the default camera
    let opened = videoio::VideoCapture::is_opened(&cam)?;
    if !opened {
        panic!("Unable to open default camera!");
    }

    let fps = cam.get(videoio::CAP_PROP_FPS)?; // Get the FPS of the camera
    println!("Camera FPS: {}", fps);

    let mut frame = Mat::default();

    let mut display_original = false;

    // The fourth parameter in the Scalar::new function represents the alpha (transparency) channel, where 0.0 means fully transparent, and 255.0 means fully opaque.

    let mut frame_count = 0;
    let mut prev_time = Instant::now();

    loop {
        cam.read(&mut frame)?;
        if frame.size()?.width <= 0 {
            break;
        }

        let frame4 = add_overlay(&frame, 5, 2.0 / 3.0)?;

        let now = Instant::now();
        let elapsed = now - prev_time;
        frame_count += 1;

        if elapsed >= Duration::from_secs(1) {
            let fps = frame_count as f64 / elapsed.as_secs_f64();
            println!("FPS: {:.2}", fps);
            prev_time = now;
            frame_count = 0;
        }

        if display_original {
            highgui::imshow(window, &frame).expect("display image");
            // highgui::imshow(window, ov).expect("display image");
        } else {
            highgui::imshow(window, &frame4).expect("display image");
        }

        let key = highgui::wait_key(10)?;
        if key == 'f' as i32 {
            display_original = !display_original;
            continue;
        }
        if key > 0 && key != 255 {
            break;
        }
    }

    highgui::destroy_all_windows()?;

    Ok(())
}

fn add_overlay(frame: &Mat, border_percentage: i32, ratio: f32) -> Result<Mat> {
    let size = frame.size()?;
    let mut frame4 = unsafe { Mat::new_size(size, CV_32FC4)? };

    cvt_color(&frame, &mut frame4, COLOR_BGR2BGRA, 4)?;

    // minus % border top
    let border_size = (size.height as f32) * border_percentage as f32 / 100.0;
    let hole_height = (size.height as f32) - 2.0 * border_size;
    // ratio ~2/3
    let hole_width = hole_height * ratio;

    let x = ((size.width as f32 / 2.0) - hole_width / 2.0) as i32;

    // println!(
    //     "rect({}, {}, {}x{})",
    //     x, border_size, hole_width, hole_height
    // );

    let hole_rect = Rect::new(x, border_size as i32, hole_width as i32, hole_height as i32);

    // overlay.roi_mut(hole_rect)?.set_scalar(white)?;

    let mut dst = Mat::new_size_with_default(size, opencv::core::CV_32FC4, Scalar::all(0.0))?;

    let mut mask = Mat::new_size_with_default(size, opencv::core::CV_32FC4, Scalar::all(0.3))?;
    opencv::imgproc::rectangle(
        &mut mask,
        hole_rect,
        Scalar::new(1.0, 1.0, 1.0, 0.0),
        -1,
        opencv::imgproc::LINE_4,
        0,
    )?;

    opencv::core::multiply(&frame4, &mask, &mut dst, 1.0, 8)?;

    Ok(dst)
}
