use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "PascalCase"))]
pub struct Part {
    pub availability: Option<String>,
    pub data_sheet_url: Option<String>,
    pub description: Option<String>,
    pub factory_stock: Option<String>,
    pub image_path: Option<String>,
    pub category: Option<String>,
    pub lead_time: Option<String>,
    pub lifecycle_status: Option<String>,
    pub manufacturer: Option<String>,
    pub manufacturer_part_number: Option<String>,
    pub mouser_part_number: Option<String>,
    #[serde(rename(deserialize = "ROHSStatus"))]
    pub rohs_status: Option<String>,
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    pub availability_in_stock: Option<u32>,
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "PascalCase"))]
pub struct ManufacturerListResponse {
    pub errors: Option<Vec<Error>>,
    pub mouser_manufacturer_list: Option<ManufacturerList>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "PascalCase"))]
pub struct ManufacturerList {
    pub count: u32,
    pub manufacturer_list: Vec<Manufacturer>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "PascalCase"))]
pub struct Manufacturer {
    pub manufacturer_name: String,
    pub manufacturer_id: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "PascalCase"))]
pub struct PartSearchResponse {
    pub number_of_result: i32,
    pub parts: Vec<Part>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "PascalCase"))]
pub struct PartSearchResult {
    pub errors: Option<Vec<Error>>,
    pub search_results: Option<PartSearchResponse>
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "PascalCase"))]
pub struct Error {
    id: i32,
    code: String,
    message: String,
    resource_key: String,
    resource_format_string: Option<String>,
    resource_format_string2: Option<String>,
    property_name: Option<String>,
}