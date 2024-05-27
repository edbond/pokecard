use anyhow::Result;
use indicatif::ProgressBar;
use my_lib::{db, models::Card};
use opencv::core::{
    DMatch, KeyPoint, MatTrait, MatTraitConst, MatTraitConstManual, Ptr, RangeTraitConst, Rect,
    Scalar, VectorToVec, CV_32F, CV_32FC4, NORM_HAMMING,
};
use opencv::features2d::{BFMatcher, DescriptorMatcherTrait, FlannBasedMatcher};
use opencv::flann::{self, IndexParamsTrait, SearchParamsTraitConst};
use opencv::imgcodecs::{imread, IMREAD_GRAYSCALE};
use opencv::imgproc::COLOR_BGR2BGRA;
use opencv::traits::Boxed;
use opencv::videoio::{VideoCaptureTrait, VideoCaptureTraitConst};
use opencv::{
    core::{Mat, Vector},
    features2d::{self, Feature2DTrait, ORB},
    imgcodecs::imdecode,
};
use opencv::{highgui, imgproc::cvt_color, videoio};
use rayon::prelude::*;
use std::time::Instant;
use tracing_subscriber::fmt;

fn vec_to_opencv_vector(data: Vec<u8>) -> Vector<u8> {
    let mut opencv_vector = Vector::new();
    opencv_vector.reserve(data.len());
    opencv_vector.extend(data);
    opencv_vector
}

fn load_cards_from_db() -> Result<Vec<Card>> {
    let mut conn = db::establish_connection();

    Ok(Card::all_cards(&mut conn))
}

fn main() -> Result<()> {
    fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();

    let cards = load_cards_from_db()?;

    let pb = ProgressBar::new(cards.len().try_into().unwrap());

    let mut t1 = Instant::now();

    let card_descriptors: Vec<(Mat, Vector<KeyPoint>, &Card)> = cards
        .par_iter()
        .fold(
            || Vec::new(),
            |mut vec, card| {
                // skip cards without image
                if card.image == None {
                    return vec;
                }

                let img = &mut imdecode(
                    &vec_to_opencv_vector(card.image.clone().unwrap()),
                    opencv::imgcodecs::IMREAD_GRAYSCALE,
                )
                .unwrap()
                .clone();

                // Create an ORB object
                let mut orb = ORB::create(
                    500,
                    1.2,
                    8,
                    31,
                    0,
                    2,
                    features2d::ORB_ScoreType::HARRIS_SCORE,
                    31,
                    20,
                )
                .expect("orb created");

                let mut keypoints = Vector::default();
                let mut orb_desc =
                    unsafe { Mat::new_rows_cols(0, 0, CV_32F) }.expect("descriptor mat created");
                let mask = Mat::default();
                orb.detect_and_compute(img, &mask, &mut keypoints, &mut orb_desc, false)
                    .expect("orb detect");

                vec.push((orb_desc, keypoints, card));

                vec
            },
        )
        .reduce(
            || vec![],
            |mut acc, x| {
                pb.inc(x.len() as u64);
                acc.extend(x);
                acc
            },
        );

    pb.finish();

    let duration = t1.elapsed();
    println!("done in {:?}", duration);

    t1 = Instant::now();
    let mut matcher = FlannBasedMatcher::new(
        &Ptr::new(flann::KDTreeIndexParams::new_def()?.into()),
        &Ptr::new(flann::SearchParams::new_1(50, 0.0, true)?),
    )?;

    println!("descs {}", card_descriptors.len());

    for (i, (desc, _, _)) in card_descriptors.iter().enumerate() {
        println!("{} {:?}", i, desc);

        matcher.add(&desc).expect("card added to index");
    }

    matcher.train().expect("train matcher");
    println!("Training done in {:?}", t1.elapsed());

    let window = "camera";
    highgui::named_window(window, highgui::WINDOW_AUTOSIZE)?;
    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?; // 0 is the default camera
    let opened = videoio::VideoCapture::is_opened(&cam)?;
    if !opened {
        panic!("Unable to open default camera!");
    }

    // Create an ORB object
    let mut orb = ORB::create(
        500,
        1.2,
        8,
        31,
        0,
        2,
        features2d::ORB_ScoreType::HARRIS_SCORE,
        31,
        20,
    )
    .expect("orb created");

    let mut frame = Mat::default();
    let mut frame_with_keypoints = frame.clone();

    // typical card ratio
    let ratio = 264.0 / 368.0;

    loop {
        cam.read(&mut frame)?;
        if frame.size()?.width <= 0 {
            break;
        }

        let t1 = Instant::now();
        let (frame_with_overlay, crop) = add_overlay(&frame, 5, ratio)?;
        println!("add overlay took {:?}", t1.elapsed());

        let mut keypoints = Vector::default();
        let mut query_desc = Mat::default();
        let mask = Mat::default();

        let t1 = Instant::now();

        orb.detect_and_compute(&crop, &mask, &mut keypoints, &mut query_desc, false)
            .expect("orb detect");

        println!("orb detect and compute took {:?}", t1.elapsed());

        // Draw keypoints on the image
        features2d::draw_keypoints(
            &mut frame_with_overlay.clone(),
            &keypoints,
            &mut frame_with_keypoints,
            opencv::core::Scalar::new(0.0, 255.0, 0.0, 0.0),
            features2d::DrawMatchesFlags::DEFAULT,
        )
        .expect("keypoints drawn");

        let mut matches = Vector::<DMatch>::new();
        matcher
            .match_(&query_desc, &mut matches, &Mat::default())
            .expect("match_ call");

        let mut matches_vec = matches.to_vec();

        matches_vec
            // .iter()
            // .filter(|m| m.distance <= 12.0)
            .sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());

        matches = matches_vec.into();

        println!(
            "Top 5 matches: {:?}",
            matches.iter().take(5).collect::<Vector<DMatch>>()
        );

        let top_match = matches.get(0)?;
        let (_, _, top_card) = card_descriptors.get(top_match.img_idx as usize).unwrap();

        println!("top card: {} {}", top_card.id, top_card.title);

        // if let Some(match_card) = best_match {
        //     println!(
        //         "best match: ({:?}) {} {}",
        //         best_score, match_card.id, match_card.title
        //     );

        //     let mut img = &mut imdecode(
        //         &vec_to_opencv_vector(match_card.image.clone().unwrap()),
        //         opencv::imgcodecs::IMREAD_GRAYSCALE,
        //     )
        //     .unwrap()
        //     .clone();

        //     features2d::draw_keypoints(
        //         &mut img.clone(),
        //         &best_keypoints.unwrap(),
        //         &mut img,
        //         opencv::core::Scalar::new(0.0, 255.0, 0.0, 0.0),
        //         features2d::DrawMatchesFlags::DEFAULT,
        //     )?;

        //     highgui::imshow(window, &img.clone()).expect("display image");

        //     // let mut match_img_display = Mat::default();
        //     // features2d::draw_matches(
        //     //     &frame,
        //     //     &keypoints,
        //     //     &img,
        //     //     &best_keypoints.unwrap(),
        //     //     &matches,
        //     //     &mut match_img_display,
        //     //     Scalar::new(0.0, 0.0, 255.0, 0.0),
        //     //     Scalar::new(0.0, 255.0, 0.0, 0.0),
        //     //     &Vector::<Mat>::new(),
        //     //     DrawMatchesFlags::NOT_DRAW_SINGLE_POINTS,
        //     // );
        // }

        if frame_with_keypoints.size()?.width <= 0 {
            continue;
        }

        highgui::imshow(window, &frame_with_keypoints).expect("display image");

        let key = highgui::wait_key(10)?;
        if key > 0 && key != 255 {
            break;
        }
    }

    Ok(())
}

fn add_overlay(frame: &Mat, border_percentage: i32, ratio: f32) -> Result<(Mat, Mat)> {
    let size = frame.size()?;
    let mut frame4 = unsafe { Mat::new_size(size, CV_32FC4)? };

    // Add alpha channel to frame
    cvt_color(&frame, &mut frame4, COLOR_BGR2BGRA, 4)?;

    // % of the border from top
    let border_size = (size.height as f32) * border_percentage as f32 / 100.0;
    let hole_height = (size.height as f32) - 2.0 * border_size;
    // ratio ~2/3
    let hole_width = hole_height * ratio;

    let x = ((size.width as f32 / 2.0) - hole_width / 2.0) as i32;

    let hole_rect = Rect::new(x, border_size as i32, hole_width as i32, hole_height as i32);

    let mut dst = Mat::new_size_with_default(size, opencv::core::CV_32FC4, Scalar::all(0.0))?;

    let mut mask = Mat::new_size_with_default(size, opencv::core::CV_32FC4, Scalar::all(0.3))?;
    opencv::imgproc::rectangle(
        &mut mask,
        hole_rect,
        Scalar::new(1.0, 255.0, 1.0, 0.0),
        -1,
        opencv::imgproc::LINE_4,
        0,
    )?;

    let crop = Mat::roi(&frame4, hole_rect)?.try_clone()?;

    opencv::core::multiply(&frame4, &mask, &mut dst, 1.0, 8)?;

    Ok((dst, crop))
}
