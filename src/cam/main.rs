use opencv::{
    core::{flip, Point},
    highgui,
    imgproc::{self, canny, cvt_color, find_contours, CHAIN_APPROX_SIMPLE, RETR_TREE},
    prelude::*,
    types::VectorOfVectorOfPoint,
    videoio, Result,
};

fn main() -> Result<()> {
    let window = "video capture";
    let result_window = "result window";
    let trackbar_name = "aperture";

    let threshold1_trackbar = "th1";
    let threshold2_trackbar = "th2";

    highgui::named_window(window, highgui::WINDOW_AUTOSIZE)?;

    highgui::named_window(result_window, 0)?;

    let mut cam = videoio::VideoCapture::new(2, videoio::CAP_ANY)?; // 0 is the default camera
    let opened = videoio::VideoCapture::is_opened(&cam)?;
    if !opened {
        panic!("Unable to open default camera!");
    }

    // let (tx, rx) = mpsc::sync_channel::<Mat>(1);
    // let receiver_handle = start_frame_processor(rx)?;

    // let mut apperture = 5;
    // let aperture_size: Option<&mut i32> = Some(&mut apperture);

    highgui::create_trackbar(trackbar_name, result_window, None, 3, None).expect("create trackbar");
    highgui::create_trackbar(threshold1_trackbar, result_window, None, 300, None)
        .expect("create trackbar");
    highgui::create_trackbar(threshold2_trackbar, result_window, None, 300, None)
        .expect("create trackbar");

    highgui::set_trackbar_pos(threshold1_trackbar, result_window, 104)?;
    highgui::set_trackbar_pos(threshold2_trackbar, result_window, 137)?;

    let mut grey = Mat::default();
    let mut edges = Mat::default();
    let mut flipped = Mat::default();
    let mut frame = Mat::default();

    let mut contours_cv = VectorOfVectorOfPoint::new();

    let mut n = 0;

    loop {
        n += 1;

        if n % 15 == 0 {
            println!("contours: {:?}", contours_cv);
        }

        cam.read(&mut frame)?;
        if frame.size()?.width > 0 {
            flip(&frame, &mut flipped, 1)?;

            // highgui::imshow(window, &frame)?;
            // tx.send(frame).expect("frame sent to processor");

            cvt_color(&flipped, &mut grey, imgproc::COLOR_BGR2GRAY, 0)
                .expect("converted to grayscale");

            // 3 5 7
            // 0 1 2
            let apperture_size =
                2 * highgui::get_trackbar_pos(trackbar_name, result_window).unwrap() + 3;

            // println!("apperture: {:?}", apperture_size);

            let threshold1 = highgui::get_trackbar_pos(threshold1_trackbar, result_window).unwrap();
            let threshold2 = highgui::get_trackbar_pos(threshold2_trackbar, result_window).unwrap();

            canny(
                &grey,
                &mut edges,
                threshold1.into(),
                threshold2.into(),
                apperture_size,
                true,
            )?;

            find_contours(
                &edges,
                &mut contours_cv,
                imgproc::RETR_LIST,
                CHAIN_APPROX_SIMPLE,
                Point::new(0, 0),
            )?;

            let _ = highgui::imshow(result_window, &edges).expect("display image");
        }

        let key = highgui::wait_key(60)?;
        if key > 0 && key != 255 {
            break;
        }
    }

    // drop(tx);
    // receiver_handle.join().expect("finish receiver");

    highgui::destroy_all_windows()?;

    Ok(())
}

// fn start_frame_processor(rx: Receiver<Mat>) -> Result<JoinHandle<()>> {
//     Ok(thread::spawn(move || {
//         while let Ok(frame) = rx.recv() {
//             println!("received frame");

//             let _ = frame;

//             let mut grey = frame.clone();

//             cvt_color(&frame, &mut grey, imgproc::COLOR_BGR2GRAY, 0)
//                 .expect("converted to grayscale");

//             let _ = highgui::imshow(result_window, &grey).expect("display grey image");
//         }
//     }))
// }
