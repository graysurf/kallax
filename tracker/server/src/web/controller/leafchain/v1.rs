use axum::{
    body,
    extract::{Extension, Json, Path},
    headers::ContentType,
    http::StatusCode,
    response::{IntoResponse, Response},
    TypedHeader,
};

use crate::web::extension::{LeafchainPeerAddressBook, LeafchainSpecList};

#[derive(Clone, Debug)]
pub enum GetChainSpecError {
    NotFound,
}

impl IntoResponse for GetChainSpecError {
    fn into_response(self) -> Response {
        let (status, body) = match self {
            Self::NotFound => (StatusCode::NOT_FOUND, body::Full::from(String::new())),
        };

        Response::builder()
            .status(status)
            .body(body::boxed(body))
            .expect("response should always build successfully")
    }
}

pub async fn get_chain_spec(
    Extension(LeafchainSpecList(list)): Extension<LeafchainSpecList>,
    Path(chain_id): Path<String>,
) -> Result<(StatusCode, TypedHeader<ContentType>, Vec<u8>), GetChainSpecError> {
    list.get(&chain_id).await.map_or(Err(GetChainSpecError::NotFound), |chain_spec| {
        Ok((StatusCode::OK, TypedHeader(ContentType::json()), chain_spec.as_ref().to_vec()))
    })
}

pub async fn get_peers(
    Extension(LeafchainPeerAddressBook(book)): Extension<LeafchainPeerAddressBook>,
    Path(chain_id): Path<String>,
) -> (StatusCode, Json<Vec<String>>) {
    (
        StatusCode::OK,
        Json(book.fetch_exposed_peers(chain_id).await.into_iter().map(|a| a.to_string()).collect()),
    )
}
