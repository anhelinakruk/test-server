use std::convert::Infallible;

use axum::{
    Router,
    body::{Body, Bytes},
    extract::Path,
    http::{HeaderMap, HeaderValue},
    response::IntoResponse,
    routing::get,
};
use futures::stream;
use hyper::{Response, StatusCode};
use models::{Account, LocalisedDescription, LocalisedDescriptionParam, Recipient, Transaction};
use serde_json::json;

pub mod models;

pub fn router() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/retail/transaction/{id}", get(get_retail_transaction))
}

pub async fn root() -> String {
    "Hello, World!".to_string()
}

pub async fn get_retail_transaction(
    headers: HeaderMap,
    Path(id): Path<String>,
) -> impl IntoResponse {
    // let user_id = headers.get("user-id");
    // let device_id = headers.get("x-device-id");
    // let cookie = headers.get("cookie");

    // if user_id.is_some() && device_id.is_some() && cookie.is_some() {
    println!("headers: {:?}", headers);
    let response = vec![Transaction {
        id: id,
        registered_identity_id: "d51ad7f6-bbf0-4a65-a2e0-e8ac80b50007".to_string(),
        leg_id: "681a5629-d335-a422-0000-141702a35f91".to_string(),
        group_key: "681a5629-d335-a422-9bf6-141702a35f91".to_string(),
        transaction_type: "TRANSFER".to_string(),
        state: "COMPLETED".to_string(),
        started_date: 1746556457011,
        updated_date: 1746556457512,
        completed_date: 1746556457511,
        created_date: 1746556457011,
        currency: "PLN".to_string(),
        amount: -2,
        amount_with_charges: -2,
        fee: 0,
        balance: 552,
        description: "To PAWEL DARIUSZ N".to_string(),
        comment: "4SQJD2faiHY".to_string(),
        tag: "transfers".to_string(),
        category: "transfers".to_string(),
        account: Account {
            id: "d77f257f-d1a1-402c-aba3-94334f415d5a".to_string(),
            account_type: "CURRENT".to_string(),
        },
        suggestions: vec![],
        cancellable: false,
        recallable: false,
        rate: 1,
        recipient: Recipient {
            id: "85dbd546-254b-4113-a785-5c68b06a51df".to_string(),
            recipient_type: "INDIVIDUAL".to_string(),
            first_name: "PAWEL DARIUSZ".to_string(),
            last_name: "NOWAK".to_string(),
            country: "PL".to_string(),
            username: "neotheprogramist".to_string(),
            code: "paweuseht".to_string(),
            account: Account {
                id: "af8b1b78-8b2b-4ce8-a157-0c7ffd1faef4".to_string(),
                account_type: "CURRENT".to_string(),
            },
        },
        localised_description: LocalisedDescription {
            key: "transaction.description.generic.name".to_string(),
            params: vec![LocalisedDescriptionParam {
                key: "name".to_string(),
                value: "PAWEL DARIUSZ N".to_string(),
            }],
        },
    }];

    let data = json!(response).to_string().into_bytes();

    let stream = stream::once(async move { Ok::<_, Infallible>(Bytes::from(data)) });

    let mut response = Response::builder()
        .status(StatusCode::OK)
        .body(Body::from_stream(stream))
        .unwrap();

    let headers = response.headers_mut();

    headers.insert(
        "date",
        HeaderValue::from_static("Sat, 24 May 2025 19:05:25 GMT"),
    );
    headers.insert(
        "content-type",
        HeaderValue::from_static("application/json;charset=utf-8"),
    );
    *response.status_mut() = StatusCode::OK;

    response
    // } else {
    //     (
    //         axum::http::StatusCode::BAD_REQUEST,
    //         Json(Vec::<Transaction>::new()),
    //     )
    // }
}
