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
pub async fn get_or_fetch(url: &str) -> Result<Vec<u8>> {
    let response = reqwest::get(url).await?;
    let data = response.bytes().await?;
    Ok(data.to_vec())
}