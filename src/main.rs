#[allow(unused)]
#[allow(dead_code)]
pub mod models;
pub mod templates;

use crate::{
    models::*,
    templates::{
        CatProductsTemplate, CategoriesTemplate, CustomerOrdersTemplate, CustomersTemplate,
        ErrorTemplate, OrderDetailsTemplate, ProductsTemplate, RootTemplate,
    },
};

use askama::Template;
use axum::{Router, extract::Extension, extract::Path, response::Html, routing::*};
use dotenv::dotenv;
use num_format::Locale;
use sqlx::PgPool;
use std::{env, net::SocketAddr};
use tracing::Level;
use tracing_subscriber::{self};

async fn root_handler() -> impl axum::response::IntoResponse {
    let tmpl = RootTemplate {};
    let rendered = tmpl.render().unwrap();
    Html(rendered)
}

async fn categories_handler(pool: Extension<PgPool>) -> impl axum::response::IntoResponse {
    match sqlx::query_as::<_, Category>(
        "Select category_id, category_name, description From categories",
    )
    .fetch_all(&*pool)
    .await
    {
        Ok(data) => {
            let tmpl = CategoriesTemplate { categories: data };
            let rendered = tmpl.render().unwrap();
            Html(rendered)
        }
        Err(e) => {
            let errtmpl = ErrorTemplate {
                errmsg: e.to_string(),
            };
            let rendered = errtmpl.render().unwrap();
            Html(rendered)
        }
    }
}

async fn category_products_handler(
    Path(catid): Path<i16>,
    pool: Extension<PgPool>,
) -> impl axum::response::IntoResponse {
    let sql = "Select product_id, product_name, quantity_per_unit, unit_price, reorder_level From products Where pr_category_id=$1";
    match sqlx::query_as::<_, Product>(sql)
        .bind(catid)
        .fetch_all(&*pool)
        .await
    {
        Ok(data) => {
            let tmpl = CatProductsTemplate { products: data };
            let rendered = tmpl.render().unwrap();
            Html(rendered)
        }
        Err(e) => {
            let errtmpl = ErrorTemplate {
                errmsg: e.to_string(),
            };
            let rendered = errtmpl.render().unwrap();
            Html(rendered)
        }
    }
}
async fn products_handler(pool: Extension<PgPool>) -> impl axum::response::IntoResponse {
    let sql = "Select product_id, product_name, quantity_per_unit, unit_price, reorder_level From products";
    match sqlx::query_as::<_, Product>(sql).fetch_all(&*pool).await {
        Ok(data) => {
            let tmpl = ProductsTemplate { products: data };
            let rendered = tmpl.render().unwrap();
            Html(rendered)
        }
        Err(e) => {
            let errtmpl = ErrorTemplate {
                errmsg: e.to_string(),
            };
            let rendered = errtmpl.render().unwrap();
            Html(rendered)
        }
    }
}

async fn customers_handler(pool: Extension<PgPool>) -> impl axum::response::IntoResponse {
    let sql = "Select customer_id, company_name, city, country From customers";
    match sqlx::query_as::<_, Customer>(sql).fetch_all(&*pool).await {
        Ok(data) => {
            let tmpl = CustomersTemplate { customers: data };
            let rendered = tmpl.render().unwrap();
            Html(rendered)
        }
        Err(e) => {
            let errtmpl = ErrorTemplate {
                errmsg: e.to_string(),
            };
            let rendered = errtmpl.render().unwrap();
            Html(rendered)
        }
    }
}

async fn customer_orders_handler(
    Path(custid): Path<String>,
    pool: Extension<PgPool>,
) -> impl axum::response::IntoResponse {
    let sql = "Select order_id, order_date, shipped_date From orders Where ord_customer_id = $1";
    match sqlx::query_as::<_, Order>(sql)
        .bind(custid)
        .fetch_all(&*pool)
        .await
    {
        Ok(data) => {
            let tmpl = CustomerOrdersTemplate { orders: data };
            let rendered = tmpl.render().unwrap();
            Html(rendered)
        }
        Err(e) => {
            let errtmpl = ErrorTemplate {
                errmsg: e.to_string(),
            };
            let rendered = errtmpl.render().unwrap();
            Html(rendered)
        }
    }
}
async fn order_details_handler(
    Path(orderid): Path<i16>,
    pool: Extension<PgPool>,
) -> impl axum::response::IntoResponse {
    let sql = "Select product_name, quantity, od_unit_price From order_details Join products on product_id = od_product_id Where od_order_id = $1";
    match sqlx::query_as::<_, OrderDetail>(sql)
        .bind(orderid)
        .fetch_all(&*pool)
        .await
    {
        Ok(data) => {
            let tmpl = OrderDetailsTemplate { orderdetails: data };
            let rendered = tmpl.render().unwrap();
            Html(rendered)
        }
        Err(e) => {
            let errtmpl = ErrorTemplate {
                errmsg: e.to_string(),
            };
            let rendered = errtmpl.render().unwrap();
            Html(rendered)
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let dburl = env::var("DATABASE_URL").expect("db url not set");
    let pool = PgPool::connect(&dburl)
        .await
        .expect("error connecting to db");
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let app = Router::new()
        .route("/", get(root_handler))
        .route("/categories", get(categories_handler))
        .route("/category/{catid}/products", get(category_products_handler))
        .route("/products", get(products_handler))
        .route("/customers", get(customers_handler))
        .route("/customer/{custid}/orders", get(customer_orders_handler))
        .route("/order/{orderid}/details", get(order_details_handler))
        .layer(Extension(pool));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:9090").await.unwrap();

    let addr = SocketAddr::from(([127, 0, 0, 1], 9090));
    println!("running on http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}
