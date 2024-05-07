use near_jsonrpc_client::{methods, JsonRpcClient};
use near_jsonrpc_primitives::types::query::QueryResponseKind;
use near_primitives::types::{AccountId, BlockReference, Finality};
use near_primitives::views::{AccountView, QueryRequest};

pub async fn fetch_data(
    url: String,
    account_string_id: String,
) -> Result<AccountView, Box<dyn std::error::Error>> {
    let client = JsonRpcClient::connect(url);

    let account_id = account_string_id.parse::<AccountId>()?;

    let request = methods::query::RpcQueryRequest {
        block_reference: BlockReference::Finality(Finality::Final),
        request: QueryRequest::ViewAccount { account_id },
    };

    let response = client.call(request).await?;

    if let QueryResponseKind::ViewAccount(account) = response.kind {
        Ok(account)
    } else {
        Err("Failed to fetch account data".into())
    }
}
