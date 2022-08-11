use poem::{Server, Route};
use poem::listener::TcpListener;
use poem_openapi::payload::{Binary, Json};
use poem_openapi::{Object, OpenApi, OpenApiService};

#[derive(Debug, Object)]
struct MyObj {
    value: i32,
}

struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/", method = "get", actual_type = "Json<MyObj>")]
    async fn test(&self) -> Binary<Vec<u8>> {
        Binary(b"{ \"value\": 100 }".to_vec())
    }
}

#[tokio::main]
async fn main() {
    let api_service = OpenApiService::new(
        Api,
        "Gateway",
        env!("CARGO_PKG_VERSION"),
    );
    let ui = api_service.rapidoc();
    let spec = api_service.spec_endpoint();

    let routes = Route::new()
                .nest("/api", api_service)
                .nest("/", ui)
                .at("/spec", spec);
    Server::new(TcpListener::bind("[::1]:3000"))
        .run(routes)
        .await.unwrap();
}
