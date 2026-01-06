use chrono::NaiveDate;
use serde::Serialize;
use sqlx::FromRow;

#[derive(Serialize, FromRow, Debug)]
pub struct Category {
    pub id: i16,
    pub name: String,
    pub descr: String,
}

#[derive(Serialize, FromRow, Debug)]
pub struct Product {
    pub id: i16,
    pub name: String,
    pub qtyperunit: String,
    pub price: f32,
    pub rorlevel: i16,
}

#[derive(Serialize, FromRow, Debug)]
pub struct Customer {
    pub id: String,
    pub name: String,
    pub city: String,
    pub country: String,
}

#[derive(Serialize, FromRow, Debug)]
pub struct Order {
    pub id: i16,
    pub orderdate: NaiveDate,
    pub shipdate: Option<NaiveDate>,
}

#[derive(Serialize, FromRow, Debug)]
pub struct OrderDetail {
    pub productname: String,
    pub quantity: i16,
    pub price: f32,
}
