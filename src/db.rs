use dotenv::dotenv;
use sqlx::{PgPool};
use std::env;

use crate::models::*;

#[derive(Clone)]
pub struct Db {
    pool: PgPool
}

impl Db {
    pub async fn connect() -> Self {
        dotenv().ok();
        let dburl = env::var("DATABASE_URL").expect("db url not set");
        let pool = PgPool::connect(&dburl).await.expect("Failed to connect");
        Self { pool }
    }

    // Direct table functions
    pub async fn get_categories(&self) -> Vec<Category> {
        sqlx::query_as::<_, Category>(
            r#"SELECT category_id as id, category_name as name, description as descr FROM categories"#
        )
        .fetch_all(&self.pool)
        .await
        .unwrap()
    }

    pub async fn get_products(&self) -> Vec<Product> {
        sqlx::query_as::<_, Product>(
            r#"SELECT product_id as id, product_name as name, quantity_per_unit as qtyperunit, unit_price as price, reorder_level as rorlevel FROM products"#
        )
        .fetch_all(&self.pool)
        .await
        .unwrap()
    }

    pub async fn get_products_by_category(&self, catid: i16) -> Vec<Product> {
        sqlx::query_as::<_, Product>(
            r#"SELECT p.product_id as id, p.product_name as name, quantity_per_unit as qtyperunit, p.unit_price as pric, p.reorder_level as rorlevel
               FROM products p JOIN categories c ON p.category_id = c.category_id WHERE c.category_id = $1"#
        )
        .bind(catid)
        .fetch_all(&self.pool)
        .await
        .unwrap()
    }

    pub async fn get_customers(&self) -> Vec<Customer> {
        sqlx::query_as::<_, Customer>(
            r#"SELECT customer_id as id, company_name as name, city, country From customers"#
        )
        .fetch_all(&self.pool)
        .await
        .unwrap()
    }

    pub async fn get_customer_orders(&self, custid: &str) -> Vec<Order> {
        sqlx::query_as::<_, Order>(
            r#"SELECT order_id as id, order_date as orderdate, shipped_date as shipdate 
               FROM orders WHERE ord_customer_id = $1"#
        )
        .bind(custid)
        .fetch_all(&self.pool)
        .await
        .unwrap()
    }

    pub async fn get_order_details(&self, orderid: i16) -> Vec<OrderDetail> {
        sqlx::query_as::<_, OrderDetail>(
            r#"SELECT product_name as productname, quantity, od_unit_price as price
            From order_details
            Join products on product_id = od_product_id
            WHERE od_order_id = $1"#
        )
        .bind(orderid)
        .fetch_all(&self.pool)
        .await
        .unwrap()
    }
}
