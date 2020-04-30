use crate::nutrients::{Carbohydrates, Energy, Fat, Protein, Salt};
use crate::products::Product;
use serde::Deserialize;
use std::error::Error;

pub fn example(path: &str) -> Result<(), Box<dyn Error>> {
    // Build the CSV reader and iterate over each record.
    // let mut rdr = csv::Reader::from_reader(io::stdin());
    let mut rdr = csv::Reader::from_path(path)?;
    for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result?;
        println!("{:?}", record.get(2));
    }
    Ok(())
}

#[derive(Debug, Deserialize)]
struct Record {
    pub name: String,
    pub manufacturer: String,
    pub kcal: f64,
    pub kj: f64,
    pub carbs: f64,
    pub fiber: f64,
    pub sugar: f64,
    pub added_sugar: f64,
    pub starch: f64,
    pub fat: f64,
    pub saturated: f64,
    pub monounsat: f64,
    pub trans: f64,
    pub protein: f64,
    pub salt: f64,
}

pub fn import_products(path: &str) -> Result<Vec<Product>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(path)?;

    let mut pass: Vec<Product> = vec![];

    for record in rdr.records() {
        let record = record?;
        // name: String,
        // manufacturer: String,
        // energy: Energy,
        // carbohydrates: Carbohydrates,
        // fat: Fat,
        // protein: Protein,
        // salt: Salt,
        let name = record.get(0).unwrap_or("");
        let manufacturer = record.get(1).unwrap_or("");

        let kcal = record.get(2).unwrap_or("0.0").parse::<f64>().unwrap_or(0.0);
        let kj = record.get(3).unwrap_or("0.0").parse::<f64>().unwrap_or(0.0);
        let carbs = record.get(4).unwrap_or("0.0").parse::<f64>().unwrap_or(0.0);
        let fiber = record.get(5).unwrap_or("0.0").parse::<f64>().unwrap_or(0.0);
        let sugar = record.get(6).unwrap_or("0.0").parse::<f64>().unwrap_or(0.0);
        let added_sugar = record.get(7).unwrap_or("0.0").parse::<f64>().unwrap_or(0.0);
        let starch = record.get(8).unwrap_or("0.0").parse::<f64>().unwrap_or(0.0);
        let fat = record.get(9).unwrap_or("0.0").parse::<f64>().unwrap_or(0.0);
        let saturated = record
            .get(10)
            .unwrap_or("0.0")
            .parse::<f64>()
            .unwrap_or(0.0);
        let monounsat = record
            .get(11)
            .unwrap_or("0.0")
            .parse::<f64>()
            .unwrap_or(0.0);
        let trans = record
            .get(12)
            .unwrap_or("0.0")
            .parse::<f64>()
            .unwrap_or(0.0);
        let protein = record
            .get(13)
            .unwrap_or("0.0")
            .parse::<f64>()
            .unwrap_or(0.0);
        let salt = record
            .get(14)
            .unwrap_or("0.0")
            .parse::<f64>()
            .unwrap_or(0.0);

        pass.push(Product::new(
            -1,
            name.clone().to_owned(),
            manufacturer.clone().to_owned(),
            Energy::new(kcal, kj),
            Carbohydrates::new(carbs, fiber, sugar, added_sugar, starch),
            Fat::new(fat, saturated, monounsat, trans),
            Protein::new(protein),
            Salt::new(salt),
        ));
    }
    Ok(pass)
}
