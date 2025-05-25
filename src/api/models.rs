use serde::Serialize;

#[derive(Serialize)]
pub struct Account {
    pub id: String,
    #[serde(rename = "type")]
    pub account_type: String,
}

#[derive(Serialize)]
pub struct Recipient {
    pub id: String,
    #[serde(rename = "type")]
    pub recipient_type: String,
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "lastName")]
    pub last_name: String,
    pub country: String,
    pub username: String,
    pub code: String,
    pub account: Account,
}

#[derive(Serialize)]
pub struct LocalisedDescriptionParam {
    pub key: String,
    pub value: String,
}

#[derive(Serialize)]
pub struct LocalisedDescription {
    pub key: String,
    pub params: Vec<LocalisedDescriptionParam>,
}

#[derive(Serialize)]
pub struct Transaction {
    pub id: String,
    #[serde(rename = "registeredIdentityId")]
    pub registered_identity_id: String,
    #[serde(rename = "legId")]
    pub leg_id: String,
    #[serde(rename = "groupKey")]
    pub group_key: String,
    #[serde(rename = "type")]
    pub transaction_type: String,
    pub state: String,
    #[serde(rename = "startedDate")]
    pub started_date: i64,
    #[serde(rename = "updatedDate")]
    pub updated_date: i64,
    #[serde(rename = "completedDate")]
    pub completed_date: i64,
    #[serde(rename = "createdDate")]
    pub created_date: i64,
    pub currency: String,
    #[serde(rename = "amount")]
    pub amount: i64,
    #[serde(rename = "amountWithCharges")]
    pub amount_with_charges: i64,
    pub fee: i64,
    pub balance: i64,
    pub description: String,
    pub comment: String,
    pub tag: String,
    pub category: String,
    pub account: Account,
    pub suggestions: Vec<String>,
    pub cancellable: bool,
    pub recallable: bool,
    pub rate: i64,
    pub recipient: Recipient,
    #[serde(rename = "localisedDescription")]
    pub localised_description: LocalisedDescription,
}
