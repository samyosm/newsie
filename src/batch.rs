use std::{
    fs::{self, File},
    io::BufReader,
};

use rayon::prelude::*;
use serde::{Deserialize, Serialize};

use crate::article::{Category, Language};

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, Debug, Default)]
pub struct Batch {
    pub language: Language,
    pub category: Category,
    pub feeds: Vec<String>,
}

pub fn get_batches() -> Vec<Batch> {
    let dir: &'static str = option_env!("batches_dir").unwrap_or("batches");
    let paths = fs::read_dir(dir).expect("error: invalid batch directory");
    paths
        .par_bridge()
        .map(|file| {
            let entry = file.expect("error: couldn't read file");
            let file = File::open(entry.path()).expect("error: couldn't open file");

            let reader = BufReader::new(file);
            let batch: Batch = serde_yaml::from_reader(reader).expect("error: couldn't parse file");
            batch
        })
        .collect()
}
