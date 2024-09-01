use reqwest::*;
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde_json::json;

pub fn worker_to_worker() -> core::result::Result<String, String> {
    let url = "https://x.api.golem.cloud/api/v3/user";

    let client =
        Client::builder().build().map_err(|e| format!("Failed to create client: {}", e))?;

    // JSON body
    let json_body = json!({
        "id": "afsalthaj",
        "name": "AfsalThaj",
        "email": "adam@golem.cloud"
    });

    let response = client
        .post(url)
        .header(ACCEPT, "application/json")
        .json(&json_body)
        .send();

    match response {
        Ok(response) => {
            if response.status().is_success() {
                let id_out_result = response.text();
                match id_out_result {
                    Ok(id_out) => {
                        Ok(id_out)
                    }
                    Err(e) => {
                        Err(format!("Failed to read response ID: {}", e))
                    }
                }
            } else {
                Err(format!("Request failed with status: {}", response.status()))
            }
        }
        Err(e) => {
            Err(format!("Request failed with error: {}", e))
        }
    }
}
