use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub errors: Vec<Value>,
    pub results: Vec<Result>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Result {
    pub aggregations: Aggregations,
    pub algorithm: String,
    pub search_type: String,
    pub total_results: i64,
    pub result_id: String,
    pub results: Vec<Result2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Aggregations {
    pub card_type: Vec<CardType>,
    pub energy_type: Option<Vec<EnergyType>>,
    pub rarity_name: Vec<RarityName>,
    pub set_name: Vec<SetName>,
    pub product_type_name: Vec<ProductTypeName>,
    pub product_line_name: Vec<ProductLineName>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardType {
    pub url_value: String,
    pub is_active: bool,
    pub value: String,
    pub count: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnergyType {
    pub url_value: String,
    pub is_active: bool,
    pub value: String,
    pub count: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RarityName {
    pub url_value: String,
    pub is_active: bool,
    pub value: String,
    pub count: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetName {
    pub url_value: String,
    pub is_active: bool,
    pub value: String,
    pub count: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductTypeName {
    pub url_value: String,
    pub is_active: bool,
    pub value: String,
    pub count: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductLineName {
    pub url_value: String,
    pub is_active: bool,
    pub value: String,
    pub count: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Result2 {
    pub shipping_category_id: f64,
    pub duplicate: bool,
    pub product_line_url_name: String,
    pub product_url_name: String,
    pub product_type_id: f64,
    pub rarity_name: String,
    pub sealed: bool,
    pub market_price: f64,
    // pub custom_attributes: CustomAttributes,
    pub lowest_price_with_shipping: f64,
    pub product_name: String,
    pub set_id: f64,
    pub product_id: f64,
    pub score: f64,
    pub set_name: String,
    pub foil_only: bool,
    pub set_url_name: String,
    pub seller_listable: bool,
    pub total_listings: f64,
    pub product_line_id: f64,
    pub product_status_id: f64,
    pub product_line_name: String,
    pub max_fulfillable_quantity: f64,
    // pub listings: Vec<Listing>,
    pub lowest_price: f64,
    pub median_price: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomAttributes {
    pub description: String,
    pub attack2: Option<String>,
    pub stage: Option<String>,
    pub detail_note: Option<String>,
    #[serde(default)]
    pub energy_type: Vec<String>,
    pub release_date: String,
    pub number: String,
    pub card_type: Vec<String>,
    pub retreat_cost: Option<String>,
    pub card_type_b: String,
    pub resistance: Option<String>,
    pub rarity_db_name: String,
    pub weakness: Option<String>,
    pub flavor_text: Option<String>,
    pub attack1: Option<String>,
    pub hp: Option<String>,
    pub attack3: Value,
    pub attack4: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Listing {
    pub direct_product: bool,
    pub gold_seller: bool,
    pub listing_id: f64,
    pub channel_id: f64,
    pub condition_id: f64,
    pub verified_seller: bool,
    pub direct_inventory: f64,
    pub ranked_shipping_price: f64,
    pub product_id: f64,
    pub printing: String,
    pub language_abbreviation: String,
    pub seller_name: String,
    pub forward_freight: bool,
    pub seller_shipping_price: f64,
    pub language: String,
    pub shipping_price: f64,
    pub condition: String,
    pub language_id: f64,
    pub score: f64,
    pub direct_seller: bool,
    pub product_condition_id: f64,
    pub seller_id: String,
    pub listing_type: String,
    pub seller_rating: f64,
    pub seller_sales: String,
    pub quantity: f64,
    pub seller_key: String,
    pub price: f64,
    pub custom_data: CustomData,
    pub listed_date: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomData {
    pub images: Vec<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub link_id: Option<String>,
}
