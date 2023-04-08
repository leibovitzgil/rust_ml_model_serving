pub mod download_util {

    use bytes::Bytes;


    type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;
    type GenericResult<T> = Result<T, GenericError>;

    pub async fn download_file_text_async(url: &str) -> GenericResult<Bytes> {
        let response = reqwest::get(url).await?;
        let content = response.bytes().await?;
        Ok(content)
    }
}