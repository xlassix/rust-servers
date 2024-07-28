use serde::{Deserialize, Serialize};
use tide::{self, http::mime::JSON, prelude::json, StatusCode};

#[derive(Debug, Deserialize, Serialize)]
struct User {
    name: String,
    age: i32,
}

async fn hello_route(req: tide::Request<()>) -> tide::Result {
    println!("{:?}", req);
    Ok("Hello world".into())
}

async fn handle_user_mock_retrieval(mut req: tide::Request<()>) -> tide::Result {
    let json_data: User = req.body_json().await?;

    if json_data.name.is_empty() {
        let mut res = tide::Response::new(StatusCode::NotAcceptable);
        res.set_body(json!({
            "error":"name cant be empty"
        }));
        res.set_content_type(JSON);
        return Ok(res);
    }

    if json_data.age <= 0 {
        let mut res = tide::Response::new(StatusCode::NotAcceptable);
        res.set_body(json!({
            "error":"Age must be greater than zero"
        }));
        res.set_content_type(JSON);
        return Ok(res);
    }

    let mut res = tide::Response::new(StatusCode::Ok);
    res.set_body(json!({
        "message":"data received successfully",
        "data":json_data
    }));
    res.set_content_type(JSON);
    return Ok(res);
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    const PORT: i32 = 8000;

    let mut app = tide::new();

    app.at("/hello").get(hello_route);
    app.at("/api").post(handle_user_mock_retrieval);

    println!("Attempting to is run server at http://127.0.0.1:{}", PORT);
    app.listen(format!("127.0.0.1:{}", PORT)).await?;

    Ok(())
}
