use anyhow::Result;
use clap::{command, Parser};
use indicatif::ProgressBar;
use keyed_priority_queue::KeyedPriorityQueue;
use my_lib::{db, models::Card};
use opencv::core::{
    AlgorithmTrait, DMatch, KeyPoint, KeyPointTrait, KeyPointTraitConst, MatTraitConst, Ptr, Rect,
    Scalar, VectorToVec, CV_32F, CV_32FC1, CV_32FC4,
};
use opencv::features2d::{DescriptorMatcherTrait, DescriptorMatcherTraitConst, FlannBasedMatcher};
use opencv::flann::{self};
use opencv::imgproc::COLOR_BGR2BGRA;
use opencv::videoio::{VideoCaptureTrait, VideoCaptureTraitConst};
use opencv::{core, highgui, imgcodecs, imgproc, objdetect, videoio, Error};
use opencv::{
    core::{Mat, Vector},
    features2d::{self, Feature2DTrait, ORB},
};
use rayon::prelude::*;
use std::collections::HashMap;
use std::time::Instant;
use tracing::{debug, info};
use tracing_subscriber::fmt;

fn vec_to_opencv_vector(data: Vec<u8>) -> Vector<u8> {
    let mut opencv_vector = Vector::new();
    opencv_vector.reserve(data.len());
    opencv_vector.extend(data);
    opencv_vector
}

fn load_cards_from_db(limit: i64) -> Result<Vec<Card>> {
    let mut conn = db::establish_connection();

    Ok(Card::all_cards(&mut conn, limit))
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    train: bool,

    #[arg(short, long)]
    cards: i64,
}

fn main() -> Result<()> {
    fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();

    let args = Args::parse();

    let mut matcher = FlannBasedMatcher::new(
        &Ptr::new(flann::KDTreeIndexParams::new(4)?.into()),
        &Ptr::new(flann::SearchParams::new_1(32, 0.0, false)?),
    )?;

    let t1 = Instant::now();

    let cards = load_cards_from_db(args.cards)?;
    let mut pb = ProgressBar::new(cards.len() as u64).with_message("loading cards from db");

    let card_descriptors: Vec<(Mat, Vector<KeyPoint>, &Card)> = cards
        .par_iter()
        .fold(
            || Vec::new(),
            |mut vec, card| {
                // skip cards without image
                if card.image == None {
                    return vec;
                }

                let img = &mut imgcodecs::imdecode(
                    &vec_to_opencv_vector(card.image.clone().unwrap()),
                    opencv::imgcodecs::IMREAD_GRAYSCALE,
                )
                .unwrap()
                .clone();

                // Create an ORB object
                let mut orb = ORB::create(
                    200,
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
    tracing::info!(duration=?duration, "done loading cards");

    if args.train {
        info!("descriptors: {}", card_descriptors.len());
        pb = ProgressBar::new(card_descriptors.len() as u64).with_message("training");

        for (desc, _, _) in card_descriptors.iter() {
            let size = desc.size()?;
            let mut d = unsafe { Mat::new_size(size, CV_32FC1)? };
            desc.convert_to(&mut d, CV_32F, 1.0 / 255.0, 0.0)?;

            matcher.add(&d).expect("card added to index");

            pb.inc(1);
        }

        pb.finish();

        matcher.train().expect("train matcher");
        info!("Training done in {:?}", t1.elapsed());
    }

    let window = "camera";
    highgui::named_window(window, highgui::WINDOW_AUTOSIZE)?;
    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?; // 0 is the default camera
    let opened = videoio::VideoCapture::is_opened(&cam)?;
    if !opened {
        panic!("Unable to open default camera!");
    }

    let mut frame = Mat::default();
    let mut frame_with_keypoints = frame.clone();

    // typical card ratio
    let ratio = 264.0 / 368.0;

    let font = imgproc::FONT_HERSHEY_SIMPLEX;
    let font_scale = 0.8;
    let font_color = Scalar::new(255.0, 255.0, 255.0, 0.0); // White color
    let thickness = 2;

    let mut capture = true;

    let mut grey = Mat::default();
    let mask = Mat::default();

    let mut top_match: Option<DMatch> = None;

    loop {
        if capture {
            cam.read(&mut frame)?;
        }

        if frame.size()?.width <= 0 {
            break;
        }

        let (frame_with_overlay, crop, hole) = add_overlay(&frame, 5, ratio)?;
        // println!("add overlay took {:?}", t1.elapsed());

        imgproc::cvt_color(&crop, &mut grey, COLOR_BGR2BGRA, 4)?;

        let mut keypoints = Vector::default();
        let mut query_desc = Mat::default();

        let t1 = Instant::now();

        // Create an ORB object
        let mut orb = ORB::create(
            200,
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

        orb.detect_and_compute(&grey, &mask, &mut keypoints, &mut query_desc, false)
            .expect("orb detect");

        // println!("orb detect and scompute took {:?}", t1.elapsed());
        // Move the keypoints by the specified offsets
        keypoints = keypoints
            .iter()
            .map(|keypoint| {
                let mut point = keypoint.pt();
                point.x += hole.x as f32;
                point.y += hole.y as f32;

                let mut k = KeyPoint::default().expect("new keypoint");
                k.set_pt(point);
                k
            })
            .collect();

        // Draw keypoints on the image
        features2d::draw_keypoints(
            &mut frame_with_overlay.clone(),
            &keypoints,
            &mut frame_with_keypoints,
            opencv::core::Scalar::new(0.0, 255.0, 0.0, 0.0),
            features2d::DrawMatchesFlags::DEFAULT,
        )
        .expect("keypoints drawn");

        let mut matches: Vector<Vector<DMatch>> = Vector::new();

        let size = query_desc.size()?;
        let mut d = unsafe { Mat::new_size(size, CV_32FC1)? };
        query_desc.convert_to(&mut d, CV_32F, 1.0 / 255.0, 0.0)?;

        matcher.knn_match(&d, &mut matches, 10, &Mat::default(), false)?;
        // matcher.radius_match(&d, &mut matches, 20.0, &Mat::default(), false)?;

        // k = 2
        // [DMatch { query_idx: 185, train_idx: 124, img_idx: 801, distance: 1.2009865 },
        //  DMatch { query_idx: 185, train_idx: 26, img_idx: 251, distance: 1.2637557 }]
        //
        // k = 4
        // [DMatch { query_idx: 193, train_idx: 178, img_idx: 172, distance: 0.84797525 },
        //  DMatch { query_idx: 193, train_idx: 128, img_idx: 606, distance: 1.1964127 },
        //  DMatch { query_idx: 193, train_idx: 11, img_idx: 550, distance: 1.3276408 },
        //  DMatch { query_idx: 193, train_idx: 11, img_idx: 998, distance: 1.3276408 }]
        info!(
            "matches {:?} {}",
            matches.iter().take(10).collect::<Vector<Vector<DMatch>>>(),
            matches.len()
        );

        // [DMatch { query_idx: 0, train_idx: 121, img_idx: 225, distance: 0.71997905 },
        //  DMatch { query_idx: 0, train_idx: 121, img_idx: 886, distance: 0.71997905 },
        //  DMatch { query_idx: 0, train_idx: 183, img_idx: 685, distance: 0.81025726 },
        //  DMatch { query_idx: 0, train_idx: 10, img_idx: 613, distance: 0.86896163 },
        //  DMatch { query_idx: 0, train_idx: 92, img_idx: 713, distance: 0.9205669 },
        //  DMatch { query_idx: 0, train_idx: 140, img_idx: 560, distance: 1.0101523 },
        //  DMatch { query_idx: 0, train_idx: 132, img_idx: 530, distance: 1.0755327 },
        //  DMatch { query_idx: 0, train_idx: 132, img_idx: 976, distance: 1.0755327 },
        //  DMatch { query_idx: 0, train_idx: 162, img_idx: 261, distance: 1.1032162 },
        //  DMatch { query_idx: 0, train_idx: 129, img_idx: 497, distance: 1.1079733 }]

        // get most img_idx
        let mut freq = HashMap::<i32, u32>::new();
        for m in matches {
            for v in m {
                *freq.entry(v.img_idx).or_insert(0) += 1;
            }
        }

        // priority queue to find most frequent img_idx
        let mut pq = KeyedPriorityQueue::<i32, u32>::new();
        for (img_idx, f) in freq.iter() {
            pq.push(*img_idx, *f);
        }

        let top_match = pq.pop();
        if let Some((top_img_idx, top_freq)) = top_match {
            info!("top match {:?}", top_match);
        }

        if let Some(top_match) = top_match {
            let (_, _, top_card) = card_descriptors.get(top_match.0 as usize).unwrap();

            // println!("top card: {} {}", top_card.id, top_card.title);

            imgproc::put_text(
                &mut frame_with_keypoints,
                top_card.title.as_str(),
                core::Point::new(10, 30),
                font,
                font_scale,
                font_color,
                thickness,
                imgproc::LINE_AA,
                false,
            )?;
        }

        info!(
            "train descriptors: {}",
            matcher.get_train_descriptors()?.len()
        );

        if frame_with_keypoints.size()?.width <= 0 {
            continue;
        }

        highgui::imshow(window, &frame_with_keypoints).expect("display image");

        let key = highgui::wait_key(10)?;
        if key == 'f' as i32 {
            capture = !capture;
            continue;
        }

        if key > 0 && key != 255 {
            break;
        }
    }

    Ok(())
}

fn add_overlay(frame: &Mat, border_percentage: i32, ratio: f32) -> Result<(Mat, Mat, Rect)> {
    let size = frame.size()?;
    let mut frame4 = unsafe { Mat::new_size(size, CV_32FC4)? };

    // Add alpha channel to frame
    imgproc::cvt_color(&frame, &mut frame4, COLOR_BGR2BGRA, 4)?;

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
        Scalar::new(1.0, 1.0, 1.0, 0.0),
        -1,
        opencv::imgproc::LINE_4,
        0,
    )?;

    let crop = Mat::roi(&frame4, hole_rect)?.try_clone()?;

    opencv::core::multiply(&frame4, &mask, &mut dst, 1.0, 8)?;

    Ok((dst, crop, hole_rect))
}
