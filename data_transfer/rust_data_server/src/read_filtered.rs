use serde::{Serialize, Deserialize};

use crate::read::ReadSamplesParams;



/// All the parameters necessary to specify what data to read.
#[derive(Serialize, Deserialize, Debug)]
pub struct ReadFilteredSamplesParams {
    source: ReadSamplesParams,
    /// 0.1 means cutoff at 10% of the sample rate.
    relative_cutoff: f32,
}