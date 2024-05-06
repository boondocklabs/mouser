use model::{Manufacturer, ManufacturerListResponse, PartSearchResult};
use reqwest::Url;
use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;
use serde_json::json;

mod model;

#[derive(Debug)]
pub struct MouserApi {
    client: reqwest::Client,
    api_key: SecretString,
    base_url: Url,
}

pub struct Search<'a> {
    api: &'a MouserApi,
}

#[derive(Default, Debug)]
pub struct Error {
    pub message: Option<String>,
    pub reqwest_error: Option<reqwest::Error>,
    pub json_error: Option<serde_json::Error>,
    pub json_text: Option<String>,
    pub mouser_errors: Option<Vec<model::Error>>,
}


impl <'a> Search<'a> {
    pub async fn part(&self, part_number: String, manufacturer: Option<u64>) -> Result<Vec<model::Part>, Error> {

        let (url, req) = if let Some(manufacturer) = manufacturer { 
            let base_url = Url::parse("https://api.mouser.com/api/v1/").unwrap();
            let url = base_url.join("search/partnumberandmanufacturer").unwrap();

            let req = json!({
                "SearchByPartMfrRequest": {
                    "manufacturerId": manufacturer,
                    "mouserPartNumber": part_number
                }
            });

            (url, req)
        } else {
            let url = self.api.base_url.join("search/partnumber").unwrap();

            let req = json!({
                "SearchByPartRequest": {
                    "mouserPartNumber": part_number
                }
            });

            (url, req)
        };

        let body = self.api.post_json(url, req).await?;
        let response = self.api.parse_response::<PartSearchResult>(&body).await?;

        if let Some(errors) = response.errors {
            if errors.len() > 0 {
                return Err(Error {
                    message: Some("Mouser API returned an error".to_string()),
                    mouser_errors: Some(errors),
                    ..Default::default()
                });
            }
        }

        Ok(response.search_results.unwrap().parts)

    }

    pub async fn manufacturer_list(&self) -> Result<Vec<Manufacturer>, reqwest::Error> {
        //let url = self.api.base_url.join("search/manufacturerlist").unwrap();
        let base_url = Url::parse("https://api.mouser.com/api/v1/").unwrap();
        let url = base_url.join("search/manufacturerlist").unwrap();

        let api_key = self.api.api_key.expose_secret();

        let response = self.api.client.get(url)
            .query(&[("apiKey", api_key)])
            .send()
            .await?;

        let response: ManufacturerListResponse = response.json().await?;
        Ok(response.mouser_manufacturer_list.unwrap().manufacturer_list)
    }
}

impl MouserApi {
    pub fn new(api_key: String) -> Self {
        let base_url = Url::parse("https://api.mouser.com/api/v2/").unwrap();
        Self {
            client: reqwest::Client::new(),
            api_key: api_key.into(),
            base_url
        }
    }

    pub async fn post_json(&self, url: Url, request: serde_json::Value) -> Result<String, Error> {

        let api_key = self.api_key.expose_secret();

        let result = self.client.post(url)
            .query(&[("apiKey", api_key)])
            .json(&request)
            .send()
            .await;

        match result {
            Ok(response) => {
                // Read the response body as text. This allows including
                // the full body later if there's a deserialization error
                match response.text().await {
                    Ok(body) => {
                        Ok(body)
                    }

                    Err(e) => {
                        Err(Error {
                            message: Some("Failed to send JSON POST request".to_string()),
                            reqwest_error: Some(e),
                            ..Default::default()
                        })
                    }
                }
            }

            Err(e) => {
                Err(Error {
                    message: Some("Failed to read response body".to_string()),
                    reqwest_error: Some(e),
                    ..Default::default()
                })
            }
        }

    }

    pub async fn parse_response<'a, T: Deserialize<'a>>(&self, body: &'a String) -> Result<T, Error> {
        let result = serde_json::from_str::<T>(body);

        match result {
            Ok(response) => {
                Ok(response)
            }

            Err(e) => {
                Err(Error {
                    message: Some("JSON decoding failed".to_string()),
                    json_error: Some(e),
                    json_text: Some(body.to_string()),
                    ..Default::default()
                })
            }
        }
    }

    pub fn search(&self) -> Search {
        Search {
            api: self
        }
    }
}