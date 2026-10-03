//! Data collection functionality for gathering stamp data from various sources.
//!
//! This module contains functions for collecting stamp data from websites
//! like StampWorld and processing the collected information.

use log::info;

pub use crate::stampworld::data_collector::CollectStampworldParams;

/// Collect stamp data from StampWorld website
pub async fn collect_stampworld_data<'a>(params: &mut CollectStampworldParams<'a>) -> Vec<String> {
    info!("commands_file_generator::data_collection::collect_stampworld_data invoked");
    crate::stampworld::collect_stampworld_data(CollectStampworldParams {
        sequencer: &mut *params.sequencer,
        ..*params
    })
    .await
}
