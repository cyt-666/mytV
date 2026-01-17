use tauri::command;
use base64::{Engine as _, engine::general_purpose};

#[command]
pub async fn get_proxied_image(url: String) -> Result<String, String> {
    let client = reqwest::Client::builder()
        .user_agent("MyTV/1.0")
        .build()
        .map_err(|e| e.to_string())?;
        
    let resp = client.get(&url).send().await.map_err(|e| e.to_string())?;
    let content_type = resp.headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("image/jpeg")
        .to_string();
        
    let bytes = resp.bytes().await.map_err(|e| e.to_string())?;
    let base64_data = general_purpose::STANDARD.encode(&bytes);
    Ok(format!("data:{};base64,{}", content_type, base64_data))
}
