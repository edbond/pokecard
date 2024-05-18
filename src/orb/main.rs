use anyhow::Result;
use indicatif::ProgressBar;
use my_lib::{db, models::Card};
use opencv::boxed_ref::BoxedRef;
use opencv::core::{
    norm, norm2, DMatch, KeyPoint, MatTrait, MatTraitConst, Range, RangeTraitConst, NORM_HAMMING,
};
use opencv::features2d::{BFMatcher, DescriptorMatcher, DescriptorMatcherTrait};
use opencv::imgcodecs::{imread, IMREAD_COLOR, IMREAD_GRAYSCALE};
use opencv::videoio::{VideoCaptureTrait, VideoCaptureTraitConst};
use opencv::{
    core::{Mat, Vector},
    features2d::{self, Feature2DTrait, ORB},
    imgcodecs::{imdecode, IMREAD_REDUCED_COLOR_8},
};
use opencv::{
    highgui,
    imgproc::{self, canny, cvt_color, find_contours, CHAIN_APPROX_SIMPLE, RETR_TREE},
    types::VectorOfVectorOfPoint,
    videoio,
};
use rayon::prelude::*;
use std::cmp::min;
use std::collections::HashMap;
use std::thread::sleep;
use std::time::{Duration, Instant};
use tracing_subscriber::fmt;

fn vec_to_opencv_vector(data: Vec<u8>) -> Vector<u8> {
    let mut opencv_vector = Vector::new();
    opencv_vector.reserve(data.len());
    opencv_vector.extend(data);
    opencv_vector
}

// fn load_images_from_db() -> Result<Vec<Mat>> {
//     let mut conn = db::establish_connection();

//     let images = Card::all_cards(&mut conn);

//     let mats = images
//         .iter()
//         .filter_map(move |i| {
//             let j = i.image.clone();
//             imdecode(&vec_to_opencv_vector(j.unwrap()), IMREAD_COLOR).ok()
//         })
//         .collect();

//     Ok(mats)
// }

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

    let t1 = Instant::now();

    let card_descriptors: Vec<(Mat, &Card)> = cards
        .par_iter()
        .fold(
            || Vec::new(),
            |mut vec, card| {
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
                let mut orb_desc = Mat::default();
                let mask = Mat::default();
                orb.detect_and_compute(img, &mask, &mut keypoints, &mut orb_desc, false)
                    .expect("orb detect");

                vec.push((orb_desc, card));

                // Draw keypoints on the image
                // features2d::draw_keypoints(
                //     &mut img.clone(),
                //     &keypoints,
                //     img,
                //     opencv::core::Scalar::new(0.0, 255.0, 0.0, 0.0),
                //     features2d::DrawMatchesFlags::DEFAULT,
                // )
                // .expect("keypoints drawn");

                // // Save the image with keypoints drawn
                // opencv::imgcodecs::imwrite(
                //     format!("output/output.{}.jpg", card.id).as_str(),
                //     img,
                //     &Vector::new(),
                // )
                // .expect("image written");

                pb.inc(1);

                vec
            },
        )
        .reduce(
            || vec![],
            |mut acc, x| {
                acc.extend(x);
                acc
            },
        );

    pb.finish();

    let duration = t1.elapsed();
    println!("done in {:?}", duration);

    let window = "camera";
    highgui::named_window(window, highgui::WINDOW_AUTOSIZE)?;
    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?; // 0 is the default camera
    let opened = videoio::VideoCapture::is_opened(&cam)?;
    if !opened {
        panic!("Unable to open default camera!");
    }

    let mut frame = Mat::default();
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

    let mut i = 0;

    frame = imread("Photo on 5-17-24 at 5.14 PM.jpg", IMREAD_GRAYSCALE)?;

    loop {
        cam.read(&mut frame)?;
        if frame.size()?.width <= 0 {
            break;
        }
        let mut frame_with_keypoints = frame.clone();

        i += 1;
        if i % 27 == 0 {
            i = 0;

            let mut keypoints = Vector::default();
            let mut query_desc = Mat::default();
            let mask = Mat::default();
            orb.detect_and_compute(&frame, &mask, &mut keypoints, &mut query_desc, false)
                .expect("orb detect");

            // Draw keypoints on the image
            features2d::draw_keypoints(
                &mut frame.clone(),
                &keypoints,
                &mut frame_with_keypoints,
                opencv::core::Scalar::new(0.0, 255.0, 0.0, 0.0),
                features2d::DrawMatchesFlags::DRAW_RICH_KEYPOINTS,
            )
            .expect("keypoints drawn");

            // # Match descriptors
            // let mut scores: HashMap<&Card, f64> = HashMap::new();

            let mut best_match: Option<&Card> = None;
            let mut best_score = f64::MAX;

            let q_size = query_desc.size()?;

            for (desc, card) in card_descriptors.iter() {
                // println!("size1 {:?}, size2 {:?}", query_desc.size()?, desc.size()?);

                // let min_size = opencv::core::min(&query_desc.size()?, &desc.size()?);

                let d_size = desc.size()?;

                let mut desc_resized: BoxedRef<Mat> = BoxedRef::from(desc.clone());
                let mut query_resized: BoxedRef<Mat> = BoxedRef::from(query_desc.clone());

                if d_size > q_size {
                    let r = Range::new(0, query_desc.rows())?;
                    desc_resized = desc.row_range(&r)?;
                } else {
                    let r = Range::new(0, desc.rows())?;
                    query_resized = query_desc.row_range(&r)?;
                }

                // println!(
                //     "sizes {:?} {:?}",
                //     query_resized.size()?,
                //     desc_resized.size()?
                // );

                let score = norm2(
                    &query_resized,
                    &desc_resized,
                    opencv::core::NORM_HAMMING,
                    &Mat::default(),
                )?;

                // println!("score {}, {}", score, card.title);

                if score < best_score {
                    best_match = Some(card);
                    best_score = score;
                }

                // scores.insert(card, score);
            }

            if let Some(match_card) = best_match {
                println!(
                    "best match: ({:?}) {} {}",
                    best_score, match_card.id, match_card.title
                );

                // let mut match_img_kps = Vector::<KeyPoint>::new();
                // let mut match_img_desc = Mat::default();

                // orb.detect_and_compute(&frame, &mask, &mut keypoints, &mut query_desc, false);

                // let mut keypoints = Vector::<KeyPoint>::default();
                // let mut query_desc = Mat::default();
                // let mask = Mat::default();

                // let match_mat = vec_to_opencv_vector(match_card.image.clone().unwrap());

                // orb.detect_and_compute(&match_mat, &mask, &mut keypoints, &mut query_desc, false)
                //     .expect("orb detect");

                // let mut matcher = BFMatcher::create(NORM_HAMMING, true)?;
                // let mut matches = Vector::<Vector<DMatch>>::new();
                // matcher
                //     .knn_match(&query_desc, &mut matches, 1, &Mat::default(), false)
                //     .unwrap();

                // let mut match_img_display = Mat::default();
                // features2d::draw_matches(
                //     &frame,
                //     &query_kps,
                //     &match_img,
                //     &match_img_kps,
                //     &matches,
                //     &mut match_img_display,
                //     Scalar::new(0.0, 0.0, 255.0, 0.0),
                //     Scalar::new(0.0, 255.0, 0.0, 0.0),
                //     &Vector::<Mat>::new(),
                //     DrawMatchesFlags::NOT_DRAW_SINGLE_POINTS,
                // );
            }
        }

        let _ = highgui::imshow(window, &frame_with_keypoints).expect("display image");

        let key = highgui::wait_key(10)?;
        if key > 0 && key != 255 {
            break;
        }
    }

    Ok(())
}

// # Load images
// image1 = cv2.imread('image1.jpg')
// image2 = cv2.imread('image2.jpg')

// # Initialize feature detector
// orb = cv2.ORB_create()

// # Find keypoints and descriptors
// kp1, des1 = orb.detectAndCompute(image1, None)
// kp2, des2 = orb.detectAndCompute(image2, None)

// # Create matcher
// bf = cv2.BFMatcher(cv2.NORM_HAMMING, crossCheck=True)

// # Match descriptors
// matches = bf.match(des1, des2)

// # Sort matches by score
// matches = sorted(matches, key=lambda x: x.distance)

// # Get image with matches drawn
// match_img = cv2.drawMatches(image1, kp1, image2, kp2, matches[:50], None)
// cv2.imshow('Matches', match_img)
