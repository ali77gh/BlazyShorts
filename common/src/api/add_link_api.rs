use super::BaseApi;
use serde::{Deserialize, Serialize};
use url::Url;


#[derive(Deserialize, Serialize)]
pub struct RequestBody{
    pub link: String,
}

#[derive(Deserialize, Serialize)]
pub struct ResponseSuccess{
    pub id: String
} 
pub struct AddLinkApi;

impl BaseApi<'_, RequestBody, ResponseSuccess> for AddLinkApi {
    fn get_path() -> String {
        "/link".to_string()
    }

    fn validate(body: &RequestBody) -> Result<(), String>{

        if body.link.len() > 100 { return Err(String::from("id len is too long")); }

        match Url::parse(&body.link) {
            Err(err) => Err(err.to_string()),
            Ok(url) => {
                let scheme = url.scheme();
                if scheme != "http" && scheme != "https" {
                    return Err(String::from("url scheme not supported")) ;
                }
                if url.host().is_none(){
                    return Err(String::from("host is empty"))
                }
                if !url.host().unwrap().to_string().contains("."){
                    return Err(String::from("host is not complete"))
                }

                Ok(())
            }
        }
    }
}
