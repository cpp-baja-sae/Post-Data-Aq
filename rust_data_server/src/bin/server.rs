use std::{net::TcpStream, thread};

use rust_data_server::{data_format::UnpackedFileDescriptor, interface, read::ReadSamplesParams};
use serde::{Deserialize, Serialize};
use websocket::{
    sync::{Client, Server, Writer},
    Message, OwnedMessage,
};

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    ListDatasets,
    DatasetDescriptor(String),
    ReadSamples(ReadSamplesParams),
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

fn as_data(message: OwnedMessage) -> Option<String> {
    if let OwnedMessage::Text(text) = message {
        Some(text)
    } else {
        None
    }
}

fn handle_request(request: Request, sender: &mut Writer<TcpStream>) {
    let payload = match request {
        Request::ListDatasets => ResponsePayload::ListDatasets(interface::list_datasets()),
        Request::DatasetDescriptor(name) => match interface::dataset_descriptor(&name) {
            Some(descriptor) => ResponsePayload::DatasetDescriptor(descriptor),
            None => ResponsePayload::Error(format!("'{}' is not a valid dataset", name)),
        },
        Request::ReadSamples(params) => match interface::read_samples(params) {
            Ok(data) => ResponsePayload::ReadSamples(data),
            Err(err) => ResponsePayload::Error(err),
        },
    };
    let response = Response {
        r#final: true,
        payload,
    };
    let data = serde_json::to_string(&response).unwrap();
    sender.send_message(&Message::text(data)).unwrap();
}

fn handle_client(connection: Client<TcpStream>) {
    let addr = connection.peer_addr().unwrap();
    let (mut receiver, mut sender) = connection.split().unwrap();
    for request in receiver
        .incoming_messages()
        .filter_map(Result::ok)
        .filter_map(as_data)
    {
        let request = serde_json::from_str(&request);
        println!("{:?}", request);
        if let Err(err) = request {
            let response = Response {
                r#final: true,
                payload: ResponsePayload::Error(err.to_string()),
            };
            let data = serde_json::to_string(&response).unwrap();
            sender.send_message(&Message::text(data)).unwrap();
        } else {
            handle_request(request.unwrap(), &mut sender);
        }
    }
    println!("Connection from {} closed.", addr);
}

pub fn main() {
    let server = Server::bind("localhost:6583").unwrap();
    println!("Server running on localhost:6583");
    for connection in server.filter_map(Result::ok) {
        println!("New connection from {:?}", connection.origin());
        let client = connection.accept().unwrap();
        thread::spawn(|| handle_client(client));
    }
}
