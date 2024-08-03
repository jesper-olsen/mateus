//    Train a logistic regression model on a classification dataset.
//    The dataset is assumed to be in a .csv.gz format file.
//    Each line is a feature vector - last value is the target value.

use linfa::prelude::*;
use linfa_logistic::LogisticRegression;
use csv::ReaderBuilder;
use flate2::read::GzDecoder;
use ndarray::{Array2, Array1, Ix1};
use std::error::Error;
use std::fs::File;
use std::io::{BufReader};
//use std::env;

fn read_dataset(fname: &str) -> Result<(Dataset<f64, String, Ix1>, Dataset<f64, String, Ix1>), Box<dyn Error>> {
    //let current_dir = env::current_dir()?;
    //println!("Current directory: {:?}", current_dir);
    let file = File::open(fname)?;
    let buf_reader = BufReader::new(file);
    let gz_decoder = GzDecoder::new(buf_reader);

    // Create a CSV reader
    let mut rdr = ReaderBuilder::new()
        //.has_headers(true)
        .from_reader(gz_decoder);

    let mut features: Vec<Vec<f64>> = Vec::new();
    let mut targets: Vec<String> = Vec::new();

    for result in rdr.records() {
        let record = result?;
        let feature_row: Vec<f64> = record.iter()
            .take(record.len() - 1)
            .map(|s| s.parse().unwrap())
            .collect();
        let target: f64 = record[record.len() - 1].parse().unwrap();

        features.push(feature_row);
        //targets.push(if target > 6.0 { "good".to_string() } else { "bad".to_string() });
        //targets.push(if target >0.5 { "good".to_string() } else { "bad".to_string() });
        targets.push(format!("{target}"));
    }

    let n_samples = features.len();
    let n_features = features[0].len();

    let features_array = Array2::from_shape_vec((n_samples, n_features), features.into_iter().flatten().collect())?;
    let targets_array = Array1::from_shape_vec(n_samples, targets)?;

    let dataset = Dataset::new(features_array, targets_array);
    let (train, valid) = dataset.split_with_ratio(0.9);

    Ok((train, valid))
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read and preprocess the winequality dataset
    let fname = "Assets/winequality-red.csv.gz";
    //let fname = "Assets/ficsgamesdb_2000_standard2000_nomovetimes_394899.pgn.csv.gz";
    let (train, valid) = read_dataset(fname)?;

    println!(
        "Fit Logistic Regression classifier with #{} training points",
        train.nsamples()
    );

    let model = LogisticRegression::default()
        .max_iterations(150)
        .fit(&train)
        .unwrap();

    let pred = model.predict(&valid);
    let cm = pred.confusion_matrix(&valid).unwrap();
    println!("{:?}", cm);

    // Calculate the accuracy and Matthew Correlation Coefficient (MCC)
    println!("accuracy {}, MCC {}", cm.accuracy(), cm.mcc());

    Ok(())
}

