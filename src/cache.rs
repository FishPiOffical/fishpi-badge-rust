use anyhow::Result;
// use lru::LruCache;
// use once_cell::sync::Lazy;
// use std::sync::Mutex;
// use std::num::NonZeroUsize;

// static IMAGE_CACHE: Lazy<Mutex<LruCache<String, Vec<u8>>>> =
//     Lazy::new(|| Mutex::new(LruCache::new(NonZeroUsize::new(100).unwrap())));

// pub async fn get_or_fetch(url: &str) -> Result<Vec<u8>> {
//     {
//         let mut cache = IMAGE_CACHE.lock().unwrap();
//         if let Some(data) = cache.get(url) {
//             return Ok(data.clone());
//         }
//     }
    
//     let response = reqwest::get(url).await?;
//     let data = response.bytes().await?;
//     let bytes = data.to_vec();
    
//     {
//         let mut cache = IMAGE_CACHE.lock().unwrap();
//         cache.put(url.to_string(), bytes.clone());
//     }
    
//     Ok(bytes)
// }
const MAX_SIZE: usize = 3 * 1024 * 1024;

pub async fn get_or_fetch(url: &str) -> Result<Vec<u8>> {
    let response = reqwest::get(url).await?;
    let data = response.bytes().await?;

    if data.len() > MAX_SIZE {
        Ok(data[..MAX_SIZE].to_vec())
    } else {
        Ok(data.to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore]
    async fn test_large_image_truncate() {
        let url = "https://picsum.photos/5000/5000";
        
        let result = get_or_fetch(url).await;
        assert!(result.is_ok());
        
        let data = result.unwrap();
        println!("Downloaded {} bytes", data.len());
        
        assert!(data.len() <= 3 * 1024 * 1024);
    }

    #[tokio::test]
    #[ignore]
    async fn test_small_image_no_truncate() {
        let url = "https://picsum.photos/200/200";
        
        let data = get_or_fetch(url).await.unwrap();
        println!("Small image: {} bytes", data.len());
        
        assert!(data.len() < 1024 * 1024); 
    }
}