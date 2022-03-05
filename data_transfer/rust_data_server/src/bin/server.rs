#[macro_use]
extern crate rocket;

use rocket::{form::Form, serde::json::Json};
use rust_data_server::{
    data_format::UnpackedFileDescriptor,
    read,
    read::ReadSamplesParams,
    read_filtered::{self, ReadFilteredSamplesParams},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    ListDatasets,
    DatasetDescriptor(String),
    ReadSamples(ReadSamplesParams),
    ReadFilteredSamples(ReadFilteredSamplesParams),
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    r#final: bool,
    payload: ResponsePayload,
}

#[derive(Serialize, Deserialize)]
pub enum ResponsePayload {
    Error(String),
    ListDatasets(Vec<String>),
    DatasetDescriptor(UnpackedFileDescriptor),
    ReadSamples(Vec<f32>),
}

#[get("/datasets")]
fn list_datasets() -> Json<Vec<String>> {
    Json(read::list_datasets())
}

#[get("/datasets/<name>")]
fn read_dataset_descriptor(name: &str) -> Result<Json<UnpackedFileDescriptor>, String> {
    let result = read::dataset_descriptor(name)
        .ok_or_else(|| format!("The specified dataset does not exist."));
    Ok(Json(result?))
}

#[get("/datasets/<name>/samples?<params>")]
fn read_samples(name: &str, params: Json<ReadSamplesParams>) -> Result<Json<Vec<f32>>, String> {
    let result = read::read_samples(name, params.0);
    Ok(Json(result?))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![list_datasets, read_dataset_descriptor, read_samples])
}
