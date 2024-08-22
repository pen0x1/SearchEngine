// Pseudo Rust code - for illustrative purposes to explain the concept

async fn fetch_user_data(user_ids: Vec<u32>) -> Result<Vec<UserData>, SomeError> {
    // Imagine this function fetches user data in batches rather than individually to reduce calls
    let batches = user_ids.chunks(10); // Example: batch size of 10
    let mut all_data = Vec::new();

    for batch in batches {
        // Perform batch request to external service/API
        let batch_data = external_api_batch_fetch(batch).await?;
        all_data.extend(batch_data);
    }

    Ok(all_data)
}