pub mod errors_declaration{

    use reqwest::{StatusCode, Response};

    pub fn check_error(data:Response) ->String{
        
        match data.status(){
            StatusCode::UNAUTHORIZED=>{
                String::from("Unauthorized")
            },
            StatusCode::REQUEST_TIMEOUT=>{
                String::from("Request Timeout")
            },
            StatusCode::BAD_GATEWAY=>{
                String::from("Bad Gateway")
            },
            StatusCode::SERVICE_UNAVAILABLE=>{
                String::from("Service Unavailable")
            },
            StatusCode::GATEWAY_TIMEOUT=>{
                String::from("Gateway Timeout")
            },
            _=>{
                String::from("Unknown Error")
            }
        }
    }

    
}