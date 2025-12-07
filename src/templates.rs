use crate::Locale;
use crate::models::Category;
use crate::models::*;
use Product;
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

#[derive(Template)]
#[template(path = "catproducts.html")] // this will look in `templates/`
pub struct CatProductsTemplate {
    pub products: Vec<Product>,
}

#[derive(Template)]
#[template(path = "products.html")] // this will look in `templates/`
pub struct ProductsTemplate {
    pub products: Vec<Product>,
}

#[derive(Template)]
#[template(path = "customers.html")] // this will look in `templates/`
pub struct CustomersTemplate {
    pub customers: Vec<Customer>,
}

#[derive(Template)]
#[template(path = "customerorders.html")] // this will look in `templates/`
pub struct CustomerOrdersTemplate {
    pub orders: Vec<Order>,
}

#[derive(Template)]
#[template(path = "orderdetails.html")] // this will look in `templates/`
pub struct OrderDetailsTemplate {
    pub orderdetails: Vec<OrderDetail>,
}
