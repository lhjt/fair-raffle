use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct NistBeacon {
    pulse: Pulse,
}

#[derive(Serialize, Deserialize)]
pub struct Pulse {
    uri: String,
    version: String,
    #[serde(rename = "cipherSuite")]
    cipher_suite: i64,
    period: i64,
    #[serde(rename = "timeStamp")]
    time_stamp: String,
    #[serde(rename = "statusCode")]
    status_code: i64,
    #[serde(rename = "signatureValue")]
    signature_value: String,
    #[serde(rename = "outputValue")]
    output_value: String,
}

// https://stackoverflow.com/a/65051530
fn get_epoch_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as u128
        * 1000
        - 200000 // Need this for some reason, otherwise NIST 404s
}

pub fn get_beacon() -> String {
    let timestamp = get_epoch_ms();
    let url = format!(
        "{}/{}",
        "https://beacon.nist.gov/beacon/2.0/pulse/time", timestamp
    );
    let result = reqwest::blocking::get(url)
        .unwrap()
        .json::<NistBeacon>()
        .unwrap();

    result.pulse.output_value
}
