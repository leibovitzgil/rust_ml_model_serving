pub mod download_util {
    use std::io::{copy, Error};
    use std::fs::File;
    use std::path::Path;
    use tempfile::Builder;
    use bytes::Bytes;


    type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;
    type GenericResult<T> = Result<T, GenericError>;

    pub async fn download_file_async(url: &str) -> GenericResult<()> {
        let tmp_dir = Builder::new().prefix("example").tempdir()?;
        let target = "https://www.rust-lang.org/logos/rust-logo-512x512.png";
        let response = reqwest::get(target).await?;

        let mut dest = {
            let fname = response
                .url()
                .path_segments()
                .and_then(|segments| segments.last())
                .and_then(|name| if name.is_empty() { None } else { Some(name) })
                .unwrap_or("tmp.bin");

            println!("file to download: '{}'", fname);
            let fname = tmp_dir.path().join(fname);
            println!("will be located under: '{:?}'", fname);
            File::create(fname)?

        };
        let content = response.text().await?;
        copy(&mut content.as_bytes(), &mut dest)?;
        Ok(())
    }

    pub async fn download_file_text_async(url: &str) -> GenericResult<Bytes> {
        let response = reqwest::get(url).await?;
        let content = response.bytes().await?;
        Ok(content)
    }
}