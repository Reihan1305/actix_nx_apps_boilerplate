use actix_web::HttpResponse;
use serde_json::{json, Value};

pub struct SignatureAccessToken;

pub fn response_builder(status_code: i64, response_code: i128, message: String, additional_info: Option<Value>) -> HttpResponse {
    let mut response = json!({
        "responseCode": response_code,
        "responseMessage": message
    });

    if let Some(Value::Object(map)) = additional_info {
        if let Some(obj) = response.as_object_mut() {
            obj.extend(map);
        }
    }

    match status_code {
        200 => HttpResponse::Ok().json(response),
        400 => HttpResponse::BadRequest().json(response),
        401 => HttpResponse::Unauthorized().json(response),
        500 => HttpResponse::InternalServerError().json(response),
        _ => HttpResponse::InternalServerError().json(json!({
            "responseCode": 500000,
            "responseMessage": "Unhandled status code"
        })),
    }
}


impl SignatureAccessToken {
    pub fn signature_success_response_builder(signature: String) -> HttpResponse {
        let data = json!({
            "signature": signature
        });
        response_builder(200, 200000, "Success".to_string(), Some(data))
    }

    pub fn signature_unauthorize_response_builder()->HttpResponse{
        let response_code = 4017300;
        let message:String = String::from("Unauthorized stringToSign");
        response_builder(401,response_code , message, None)
    }
}
