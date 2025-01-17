use std::time::Duration;

use crate::asset_canister::method_names::CREATE_CHUNK;
use crate::asset_canister::protocol::{CreateChunkRequest, CreateChunkResponse};
use crate::retryable::retryable;
use crate::semaphores::Semaphores;
use anyhow::bail;
use backoff::backoff::Backoff;
use backoff::ExponentialBackoffBuilder;
use candid::{Decode, Nat};
use ic_utils::Canister;

pub(crate) async fn create_chunk(
    canister: &Canister<'_>,
    batch_id: &Nat,
    content: &[u8],
    semaphores: &Semaphores,
) -> anyhow::Result<Nat> {
    let _chunk_releaser = semaphores.create_chunk.acquire(1).await;
    let batch_id = batch_id.clone();
    let args = CreateChunkRequest { batch_id, content };
    let mut retry_policy = ExponentialBackoffBuilder::new()
        .with_initial_interval(Duration::from_secs(1))
        .with_max_interval(Duration::from_secs(16))
        .with_multiplier(2.0)
        .with_max_elapsed_time(Some(Duration::from_secs(300)))
        .build();

    loop {
        let builder = canister.update_(CREATE_CHUNK);
        let builder = builder.with_arg(&args);
        let request_id = {
            let _releaser = semaphores.create_chunk_call.acquire(1).await;
            builder
                .build()
                .map(|result: (CreateChunkResponse,)| (result.0.chunk_id,))
                .call()
                .await?
        };
        let wait_result = {
            let _releaser = semaphores.create_chunk_wait.acquire(1).await;
            canister.wait(request_id).await
        };

        match wait_result {
            Ok(response) => {
                // failure to decode the response is not retryable
                return Ok(Decode!(&response, CreateChunkResponse)?.chunk_id);
            }
            Err(agent_err) if !retryable(&agent_err) => {
                bail!(agent_err);
            }
            Err(agent_err) => match retry_policy.next_backoff() {
                Some(duration) => tokio::time::sleep(duration).await,
                None => bail!(agent_err),
            },
        }
    }
}
