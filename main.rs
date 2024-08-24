use std::collections::HashSet;
use futures::future::try_join_all;

async fn fetch_user_data(user_ids: Vec<u32>) -> Result<Vec<UserData>, SomeError> {
    let user_ids: HashSet<u32> = user_ids.into_iter().collect();
    let futures = user_ids.into_iter().collect::<Vec<_>>().chunks(10).map(|batch| {
        let batch = batch.to_vec(); 
        async move { external_api_batch_fetch(&batch).await }
    });

    let results = try_join_all(futures).await?;
    let all_data = results.into_iter().flat_map(|data| data.into_iter()).collect::<Vec<_>>();

    Ok(all_data)
}

async fn external_api_batch_fetch(batch: &[u32]) -> Result<Vec<UserData>, SomeError> {
    unimplemented!()
}