use askama::Template;
use axum:: {
    routing::get,
    Router
};
use axum::response::{ IntoResponse, Html };



#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate<'a> {
    name: & 'a str
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async {
        let hello = HelloTemplate { name: "Parth" };
        Html(hello.render().unwrap()).into_response()
    }));    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}