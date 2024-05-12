use std::time::Instant;

use anyhow::Result;
use my_lib::db;
use opencv::{
    core::{Mat, Vector},
    features2d::{self, Feature2DTrait, ORB},
    imgcodecs::imread_def,
};

fn load_images_from_db() -> Result<Vec<Mat>> {
    let conn = db::establish_connection();

    let images = Vec::<Mat>::new();

    Ok(images)
}

fn main() -> Result<()> {
    let images = load_images_from_db();

    let mut img = imread_def("IMG_1182.jpeg")?;

    let start = Instant::now();

    // Create an ORB object
    let mut orb = ORB::create(
        1000,
        1.2,
        8,
        31,
        0,
        2,
        features2d::ORB_ScoreType::HARRIS_SCORE,
        31,
        20,
    )?;

    let mut keypoints = Vector::default();
    let mut orb_desc = Mat::default();
    let mask = Mat::default();
    orb.detect_and_compute(&img, &mask, &mut keypoints, &mut orb_desc, false)?;

    let duration = start.elapsed();
    println!("Time elapsed in executing function is: {:?}", duration);

    // Draw keypoints on the image
    features2d::draw_keypoints(
        &mut img.clone(),
        &keypoints,
        &mut img,
        opencv::core::Scalar::new(0.0, 255.0, 0.0, 0.0),
        features2d::DrawMatchesFlags::DEFAULT,
    )?;

    // Save the image with keypoints drawn
    opencv::imgcodecs::imwrite("output.jpg", &img, &Vector::new())?;

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
