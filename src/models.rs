use chrono::NaiveDate;
use serde::Serialize;
use sqlx::FromRow;

#[derive(Serialize, FromRow, Debug)]
pub struct Category {
    pub category_id: i16,
    pub category_name: String,
    pub description: String,
}

#[derive(Serialize, FromRow, Debug)]
pub struct Product {
    pub product_id: i16,
    pub product_name: String,
    pub quantity_per_unit: String,
    pub unit_price: f32,
    pub reorder_level: i16,
}

#[derive(Serialize, FromRow, Debug)]
pub struct Customer {
    pub customer_id: String,
    pub company_name: String,
    pub city: String,
    pub country: String,
}

#[derive(Serialize, FromRow, Debug)]
pub struct Order {
    pub order_id: i16,
    pub order_date: NaiveDate,
    pub shipped_date: Option<NaiveDate>,
}

#[derive(Serialize, FromRow, Debug)]
pub struct OrderDetail {
    pub product_name: String,
    pub quantity: i16,
    pub od_unit_price: f32,
}
