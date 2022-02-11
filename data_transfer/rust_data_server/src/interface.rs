use std::{
    io::Read,
    sync::mpsc::{self, Receiver},
    thread,
};

use serde::{Deserialize, Serialize};

use crate::{
    ingestion::{self},
    util::Ignorable,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StatusMessage {
    IngestionProgress {
        name: String,
        completed: u64,
        total: u64,
    },
    IngestionComplete {
        name: String,
    },
}

pub fn ingest(
    input: impl Read + Send + 'static,
    size: u64,
    name: String,
) -> Receiver<StatusMessage> {
    let (sender, receiver) = mpsc::channel();
    thread::spawn(move || {
        let input = input;
        let status_sender = |completed, total| {
            sender
                .send(StatusMessage::IngestionProgress {
                    name: name.clone(),
                    completed,
                    total,
                })
                .ignore()
        };
        ingestion::ingest(input, size, &name, status_sender);
        sender
            .send(StatusMessage::IngestionComplete { name: name.clone() })
            .ignore();
    });
    receiver
}

pub use super::read::*;
