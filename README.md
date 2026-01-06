# rsaxumapp

## Rust Demo App using **Axum** framework + **Askama** templates

- *Axum* is a lightweight Rust web framework that makes routing and building web applications easy.
- *Askama* is an easy-to-use templating library for rendering HTML output in Rust.

## Key Features

- Simple routing using **Axum**
- Database connection with **PostgreSQL**
- HTML rendering using **Askama** templates
- Lightweight MIS-style data display
- Demo-ready for learning or extension

## Examples

### Routing

```rust
use axum::{routing::get, Router};

async fn hello() -> &'static str {
    "Hello, Axum!"
}

let app = Router::new().route("/", get(hello));
```

#### Database connection and starting the app

```rust
#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let d = db::Db::connect().await;

    let app = Router::new()
        .route("/", get(root_handler))
        .route("/categories", get(categories_handler))
        .route("/category/{catid}/products", get(category_products_handler))
        .route("/products", get(products_handler))
        .route("/customers", get(customers_handler))
        .route("/customer/{custid}/orders", get(customer_orders_handler))
        .route("/order/{orderid}/details", get(order_details_handler))
        .route("/zonetimes", get(zones_handler))
        .route("/zonetime", post(zone_handler))
        .layer(Extension(d));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:9090").await.unwrap();

    let addr = SocketAddr::from(([127, 0, 0, 1], 9090));
    println!("running on http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}
```

#### Route handlers example:

```rust
async fn root_handler() -> impl axum::response::IntoResponse {
    let tmpl = RootTemplate {};
    let rendered = tmpl.render().unwrap();
    Html(rendered)
}

- the handler now uses custom Db module

async fn categories_handler(db: Extension<db::Db>) -> impl axum::response::IntoResponse {
    let data = db.get_categories().await;
    let tmpl = CategoriesTemplate {categories: data};
    Html( tmpl.render().unwrap())
}
```

### Askama templates definition:

```rust
use askama_derive::Template;
#[derive(Template)]
#[template(path = "error.html")] // Error page template
pub struct ErrorTemplate {
    pub errmsg: String,
}

#[derive(Template)]
#[template(path = "index.html")] // Error page template
pub struct RootTemplate {}

#[derive(Template)]
#[template(path = "categories.html")] // Error page template
pub struct CategoriesTemplate {
    pub categories: Vec<Category>,
}
```

## Getting Started

```rust
1. Clone the repo  
2. Set `DATABASE_URL` in a `.env` file  
3. Run: `cargo run`  
4. Open your browser at `http://127.0.0.1:9090`
```

## Finally the dependencies config:

`Cargo.toml`

```rust
[dependencies]
axum = { version = "0.8", features = ["macros"] } 
askama = "0.13"
askama_derive = "0.13"
dotenv = "0.15.0"
hyper = "1.8.1"
serde = { version = "1.0.228", features = ["derive"] }
sqlx = { version = "0.8.6", features = ["postgres", "chrono", "runtime-tokio-rustls"] }
tokio = { version = "1", features = ["full", "rt-multi-thread"] }
tower = "0.5.2"
tracing = "0.1.41"
tracing-subscriber = "0.3.20"
filters = "0.4.0"
chrono = { version = "0.4.42", features = ["serde"] }
num-format = "0.4.4"
```
