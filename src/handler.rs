use crate::{
    model::{Items, Product},
    schema::Parameters,
    AppState,
};

use handlebars::Handlebars;
use serde_json::json;
use actix_web::{get, http::{header::ContentType, StatusCode},
                middleware::{ErrorHandlerResponse, ErrorHandlers},
                dev::ServiceResponse, body::BoxBody,
                web, HttpResponse, HttpRequest, Result,
};


#[get("/search")]
async fn search(hb: web::Data<Handlebars<'_>>, req: HttpRequest, data: web::Data<AppState>) -> HttpResponse {
    let info = web::Query::<Parameters>::from_query(req.query_string()).unwrap();
    println!("{:?}", info.q);

    let get = "SELECT title, price, currency, image_url, origin_url, last_update, tags FROM product";
    let query = sqlx::query_as::<_, Product>(get);
    let products = query.fetch_all(&data.db).await.unwrap();
    let items = Items {
        items: products,
    };

    let body = hb.render("result", &items).unwrap();
    HttpResponse::Ok().body(body)
}


// Custom error handlers, to return HTML responses when an error occurs.
pub fn error_handlers() -> ErrorHandlers<BoxBody> {
    ErrorHandlers::new().handler(StatusCode::NOT_FOUND, not_found)
}

// Error handler for a 404 Page not found error.
fn not_found<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<BoxBody>> {
    let response = get_error_response(&res, "Page not found");
    Ok(ErrorHandlerResponse::Response(ServiceResponse::new(
        res.into_parts().0,
        response.map_into_left_body(),
    )))
}

// Generic error handler.
fn get_error_response<B>(res: &ServiceResponse<B>, error: &str) -> HttpResponse<BoxBody> {
    let request = res.request();

    // Provide a fallback to a simple plain text response in case an error occurs during the
    // rendering of the error page.
    let fallback = |e: &str| {
        HttpResponse::build(res.status())
            .content_type(ContentType::plaintext())
            .body(e.to_string())
    };

    let hb = request
        .app_data::<web::Data<Handlebars>>()
        .map(|t| t.get_ref());
    match hb {
        Some(hb) => {
            let data = json!({
                "error": error,
                "status_code": res.status().as_str()
            });
            let body = hb.render("error", &data);

            match body {
                Ok(body) => HttpResponse::build(res.status())
                    .content_type(ContentType::html())
                    .body(body),
                Err(_) => fallback(error),
            }
        }
        None => fallback(error),
    }
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("")
        .service(search)
        .service(actix_files::Files::new("/", "./static/templates").index_file("index.html"));

    conf.service(scope);
}