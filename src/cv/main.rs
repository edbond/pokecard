use opencv::{
    core::{Point, Size, Vector},
    gapi::{self, bgr2_gray, blur_def, canny, flip, resize, threshold_1, GMat, Image},
    highgui,
    imgproc::INTER_LINEAR,
    prelude::*,
    Result,
};

fn main() -> Result<()> {
    let result_window = "result window";
    let trackbar_name = "aperture";

    let threshold1_trackbar = "th1";
    let threshold2_trackbar = "th2";

    highgui::named_window(result_window, 0)?;

    highgui::create_trackbar(trackbar_name, result_window, None, 3, None).expect("create trackbar");
    highgui::create_trackbar(threshold1_trackbar, result_window, None, 300, None)
        .expect("create trackbar");
    highgui::create_trackbar(threshold2_trackbar, result_window, None, 300, None)
        .expect("create trackbar");

    highgui::set_trackbar_pos(threshold1_trackbar, result_window, 104)?;
    highgui::set_trackbar_pos(threshold2_trackbar, result_window, 137)?;

    // let mut contours_cv = VectorOfVectorOfPoint::new();

    // cvt_color(&blurred, &mut grey, imgproc::COLOR_BGR2GRAY, 0).expect("converted to grayscale");

    loop {
        // 3 5 7
        // 0 1 2
        let aperture_size =
            2 * highgui::get_trackbar_pos(trackbar_name, result_window).unwrap() + 3;

        // println!("apperture: {:?}", apperture_size);

        let threshold1 = highgui::get_trackbar_pos(threshold1_trackbar, result_window).unwrap();
        let threshold2 = highgui::get_trackbar_pos(threshold2_trackbar, result_window).unwrap();

        let frame = opencv::imgcodecs::imread_def("IMG_1182.jpeg")?;

        let image = Image::new(Point::new(0, 0), &frame, &Mat::default())?;

        let img = GMat::default()?;
        let downsized = resize(&img, Size::default(), 0.3, 0.3, INTER_LINEAR)?;
        let flipped = flip(&downsized, 1)?;
        let blurred = blur_def(&flipped, Size::new(8, 8))?;

        let gray = bgr2_gray(&blurred)?;

        let edges = canny(
            &gray,
            threshold1 as f64,
            threshold2 as f64,
            aperture_size,
            true,
        )?;

        let mut ac = gapi::GComputation::new(img, edges)?;
        let mut output_frame = Mat::default();

        // highgui::imshow(window, &frame)?;
        // tx.send(frame).expect("frame sent to processor");

        // threshold(&grey, &mut edges, threshold1.into(), THRESH_BINARY)?;

        // println!("threshold {:?}", edges);

        // canny(
        //     &grey,
        //     &mut edges,
        //     threshold1.into(),
        //     threshold2.into(),
        //     apperture_size,
        //     false,
        // )?;

        // find_contours(
        //     &edges,
        //     &mut contours_cv,
        //     imgproc::RETR_LIST,
        //     CHAIN_APPROX_SIMPLE,
        //     Point::new(0, 0),
        // )?;

        ac.apply_2(image.img(), &mut output_frame, Vector::new())?;

        let _ = highgui::imshow(result_window, &output_frame).expect("display image");

        let key = highgui::wait_key(30)?;
        if key > 0 && key != 255 {
            break;
        }
    }

    highgui::destroy_all_windows()?;

    Ok(())
}
