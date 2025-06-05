use reqwest::{Client, ClientBuilder, Error, Response, header};
use serde::Serialize;
use serde::de::DeserializeOwned;

pub struct HttpClient(Client);

impl HttpClient {
    pub fn init() -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::USER_AGENT,
            header::HeaderValue::from_static("Mozilla/5.0 (Windows NT 12.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/70.0.3538.102 Safari/537.36 Edge/18.18362")
        );

        HttpClient(
            ClientBuilder::new()
                .default_headers(headers) // 默认 headers
                .build()
                .expect("无法创建 reqwest 客户端"),
        )
    }

    /// 获取内部 reqwest 客户端引用（兼容原生 API 调用）
    pub fn client(&self) -> &Client {
        &self.0
    }

    /// 封装 GET 请求
    pub async fn get(&self, url: &str) -> Result<Response, Error> {
        self.0.get(url).send().await
    }

    /// 封装 POST 请求
    pub async fn post<T: Serialize + ?Sized>(&self, url: &str, body: &T, ) -> Result<Response, Error> {
        self.0.post(url).json(body).send().await
    }
    
    ///  封装 GET 请求并解析为 JSON
    /// 
    /// ```
    /// let request = http.get_json(url).await?;
    /// ```
    /// 
    pub async fn get_json(&self, url: &str) -> Result<serde_json::Value, Error> {
         self.get(url).await?.json::<serde_json::Value>().await
    }
    
    
    
}
