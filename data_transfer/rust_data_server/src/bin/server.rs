#[macro_use]
extern crate rocket;

use rocket::{fairing::Fairing, form::Form, response::status, serde::json::Json};
use rocket_cors::CorsOptions;
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

#[post("/datasets/import")]
fn import_dataset() -> () {}

#[get("/datasets/<name>")]
fn read_dataset_descriptor(
    name: &str,
) -> Result<Json<UnpackedFileDescriptor>, status::NotFound<String>> {
    let result = read::dataset_descriptor(name)
        .ok_or_else(|| status::NotFound(format!("The specified dataset does not exist.")));
    Ok(Json(result?))
}

#[get("/datasets/<name>/samples?<params>")]
fn read_samples(
    name: &str,
    params: Json<ReadSamplesParams>,
) -> Result<Json<Vec<f32>>, status::NotFound<String>> {
    let result = read::read_samples(name, params.0);
    Ok(Json(result.map_err(|e| status::NotFound(e))?))
}

#[get("/datasets/<name>/filtered_samples?<params>")]
fn read_filtered_samples(
    name: &str,
    params: Json<ReadFilteredSamplesParams>,
) -> Result<Json<Vec<f32>>, status::NotFound<String>> {
    let result = read_filtered::read_filtered_samples(name, params.0);
    Ok(Json(result.map_err(|e| status::NotFound(e))?))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(
            CorsOptions {
                ..Default::default()
            }
            .to_cors()
            .unwrap(),
        )
        .mount(
            "/",
            routes![
                list_datasets,
                read_dataset_descriptor,
                read_samples,
                read_filtered_samples
            ],
        )
}
