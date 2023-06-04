use serde::{Deserialize, Serialize};

pub trait BaseApi<'a, REQ,RES> 
    where REQ: Deserialize<'a> + Serialize,
          RES: Deserialize<'a> + Serialize {
    
    // implements in common package
    fn get_path()->String;
    fn validate(request_body: &REQ) -> Result<(), String>;

    // used in server side
    fn parse_req(request_body: &'a str) -> Result<REQ, serde_json::Error>{
        serde_json::from_str(request_body)
    }
    fn parse_and_validate(request_body: &'a str)-> Result<REQ, String>{
        match Self::parse_req(request_body) {
            Err(e) => Err(e.to_string()),
            Ok(v) => {
                match Self::validate(&v) {
                    Err(e) => Err(e),
                    Ok(_) => Ok(v),
                }
            }
        }
    }
    fn create_res(resp: RES) -> String{
        serde_json::to_string(&resp).unwrap()
    }

    // used in client side
    fn parse_res(request_body: &'a str) -> Result<RES, serde_json::Error>{
        serde_json::from_str(request_body)
    }
    fn create_req(req: REQ) -> Result<String,String>{
        match Self::validate(&req) {
            Err(e) => Err(e),
            Ok(_) => Ok(serde_json::to_string(&req).unwrap())
        }
    }
}