use serde::{self,Deserialize};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Country {
    pub id: String,
    pub name: String,
    pub capital_city: String,
    pub longitude: String,
    pub latitude: String
}
