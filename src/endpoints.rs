use axum::{Json, response::{Html, IntoResponse, Response}, http::StatusCode};
use serde::{Serialize, Deserialize};

use askama::Template;
// PAGE ONE
#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate;
pub async fn get_index() -> impl IntoResponse {
    let template = HelloTemplate {};
    HtmlTemplate(template)
}

// PAGE TWO
#[derive(Template)]
#[template(path = "another_page.html")]
struct AnotherPage;
pub async fn get_page_two() -> impl IntoResponse {
    let template = AnotherPage{};
    HtmlTemplate(template)
}


pub async fn post_data(Json(payload):Json<CreateMessage>) -> Json<Message>{
    let message = Message{
        id: 1,
        message: payload.message.to_string(),
    };
    Json(message)
}

#[derive(Serialize)]
pub struct Message {
    id: i32,
    message: String,
}

#[derive(Deserialize)]
pub struct CreateMessage {
    message: String,
}

struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        // Attempt to render the template with askama
        match self.0.render() {
            // If we're able to successfully parse and aggregate the template, serve it
            Ok(html) => Html(html).into_response(),
            // If we're not, return an error or some bit of fallback HTML
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}

