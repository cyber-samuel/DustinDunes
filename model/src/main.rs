#![feature(slice_flatten)]

use std::fs;
use std::iter;

use image::io::Reader as ImageReader;
use image::GenericImageView;
use image::DynamicImage;
use ndarray::{Array2, Array, array};
use rusty_machine::learning::nnet::{NeuralNet, BCECriterion};
use rusty_machine::learning::toolkit::regularization::Regularization;
use rusty_machine::learning::optim::grad_desc::StochasticGD;
use rusty_machine::linalg::Matrix;
use rusty_machine::learning::SupModel;
use rand::seq::SliceRandom;

fn main() {
    let mut rng = &mut rand::thread_rng();

    let mut all_images = decode_images("./data/Images");
    let mut images: Vec<DynamicImage> = all_images.choose_multiple(&mut rng, 30).cloned().collect();
    let len = images.len();
    let pixel_count = images[0].height() * images[0].width();
    println!("Uploaded {} images", len);

    // create a neural net
    let layers: &[usize] = &[pixel_count.try_into().unwrap(), 109512, 2];
    let criterion = BCECriterion::new(Regularization::L2(0.1));
    let mut model = NeuralNet::new(layers, criterion, StochasticGD::default());

    let inputs = convert_format(&mut images);
    let mut data = Matrix::new(len, pixel_count.try_into().unwrap(), inputs);
    // let mut labels = Matrix::new()

    // train network
    // model.train(&data, &labels).unwrap();
    // // testing
    // let outputs = model.predict(&data).unwrap();
}

fn convert_format(data: &mut Vec<DynamicImage>) -> Vec<u8> {
    let mut flattened = vec![];
    // data.iter().map(|img| img.pixels().map(|p| p.2.0));

    for img in data {
        img.pixels().for_each(|x| flattened.push(x.2.0));
        println!("{:?}", flattened);
    }

    // data.iter().flatten().to_vec()
    flattened.flatten().to_vec()
}

fn decode_images(path: &str) -> Vec<DynamicImage> {
    let mut images = vec![]; // flattened matrix of decoded images
    let mut labels: Vec<u8> = iter::repeat(1).take(335).collect();
    let mut healthy_labels: Vec<u8> = iter::repeat(0).take(330).collect();
    labels.append(&mut healthy_labels);

    let mut paths: Vec<_> = fs::read_dir(path).unwrap().map(|r| r.unwrap()).collect();
    paths.sort_by_key(|dir| dir.path());

    for (i, path) in paths.into_iter().enumerate() {
        // println!("{:?}", path.path());
        let img = ImageReader::open(path.path())
            .unwrap()
            .decode()
            .unwrap();
        // println!("{:?}", img.dimensions());

        if img.dimensions().0 == 624 {
            images.push(img.rotate90());
        } else if img.dimensions().0 == 351 {
            images.push(img);
        } else {
            // remove label
            labels.remove(i);
            println!("Removed {}", path.path().display());
        }
    }

    images
}


