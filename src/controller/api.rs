use rocket_contrib::{Json, Value};

#[get("/book")]
pub fn book() -> Json<Value> {
    Json(json!(
        [
            { 
                "type": "success", 
                "value": { 
                    "id": 469, 
                    "joke": "Chuck Norris can unit test entire applications with a single assert.", 
                    "categories": ["nerdy"] 
                } 
            }
        ]
        
    ))
}