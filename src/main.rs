#[allow(unused)]
#[allow(dead_code)]
pub mod models;
pub mod templates;

use crate::{
    models::*, templates::{
        CatProductsTemplate, CategoriesTemplate, CustomerOrdersTemplate, CustomersTemplate,
        ErrorTemplate, OrderDetailsTemplate, ProductsTemplate, RootTemplate, ZoneTimeTemplate,
        ZoneTimesTemplate,
    }
};
use askama::Template;
use axum::{
    Router,
    extract::Extension,
    extract::{Form, Path},
    response::Html,
    routing::*,
};
use chrono::prelude::*;
use chrono_tz::Tz;
use dotenv::dotenv;
use serde::Deserialize;
use sqlx::{PgPool};
use std::{env, net::SocketAddr};
use tracing::Level;
use tracing_subscriber::{self};

async fn root_handler() -> impl axum::response::IntoResponse {
    let tmpl = RootTemplate {};
    let rendered = tmpl.render().unwrap();
    Html(rendered)
}

async fn categories_handler(db: Extension<db::Db>) -> impl axum::response::IntoResponse {
    let data = db.get_categories().await;
    let tmpl = CategoriesTemplate {categories: data};
    Html( tmpl.render().unwrap())
}

async fn category_products_handler(
    Path(catid): Path<i16>,
    db: Extension<db::Db>
) -> impl axum::response::IntoResponse {
    let data = db.get_products_by_category(catid).await;
    let tmpl = CatProductsTemplate { products: data };
    Html( tmpl.render().unwrap())
}

async fn products_handler(db: Extension<db::Db>) -> impl axum::response::IntoResponse {
    let data = db.get_products().await;
    let tmpl = ProductsTemplate { products: data };
    let rendered = tmpl.render().unwrap();
    Html(rendered)
}

async fn customers_handler(db: Extension<db::Db>) -> impl axum::response::IntoResponse {
    let data = db.get_customers().await;
    let tmpl = CustomersTemplate { customers: data };
    let rendered = tmpl.render().unwrap();
    Html(rendered)
}

async fn customer_orders_handler(
    Path(custid): Path<String>,
    db: Extension<db::Db>,
) -> impl axum::response::IntoResponse {
    let data = db.get_customer_orders(&custid).await;
    let tmpl = CustomerOrdersTemplate { orders: data };
    let rendered = tmpl.render().unwrap();
    Html(rendered)
}

async fn order_details_handler(
    Path(orderid): Path<i16>,
    db: Extension<db::Db>,
) -> impl axum::response::IntoResponse {
    let data = db.get_order_details(orderid).await;
    let tmpl = OrderDetailsTemplate { orderdetails: data };
    let rendered = tmpl.render().unwrap();
    Html(rendered)
}

async fn zones_handler() -> impl axum::response::IntoResponse {
    let data = vec![
        "America/Los_Angeles".to_string(),
        "America/New_York".to_string(),
        "Europe/London".to_string(),
        "Europe/Paris".to_string(),
        "Asia/Kolkata".to_string(),
        "Asia/Tokyo".to_string(),
        "Australia/Perth".to_string(),
        "Australia/Sydney".to_string(),
        "Pacific/Auckland".to_string(),
    ];
    let tmpl = ZoneTimesTemplate {
        zones: data,
        selected_zone: Some("Asia/Kolkata".to_string()),
    };
    let rendered = tmpl.render().unwrap();
    Html(rendered)
}

#[derive(Deserialize)]
struct ZoneForm {
    zone: String,
}

async fn zone_handler(Form(form): Form<ZoneForm>) -> Html<String> {
    let tz: Tz = form.zone.parse().unwrap();
    let utc_now = Utc::now();
    let zone_now = utc_now.with_timezone(&tz);

    let hour_angle = (zone_now.hour() as f64 * 30.0) + (zone_now.minute() as f64 / 2.0) - 90.0;
    let minute_angle = (zone_now.minute() as f64 * 6.0 + zone_now.second() as f64 / 10.0) - 90.0;
    let second_angle = (zone_now.second() as f64 * 6.0) - 90.0;

    let hr = zone_now.hour();
    let clr = match hr {
        0..6 => "#666",
        6..12 => "blue",
        12..18 => "seagreen",
        18..21 => "navy",
        _ => "#333"
    };
    let tmpl = ZoneTimeTemplate {
        zone_time: zone_now.format("%a %d %b %H:%M %p %z %Z").to_string(),
        zone: form.zone.to_string(),
        hour_angle: hour_angle,
        minute_angle: minute_angle,
        second_angle: second_angle,
        handclr: clr.to_string()
    };

    Html(tmpl.render().unwrap())
}

mod db;

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
