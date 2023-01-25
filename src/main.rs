use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::{ops::Deref, sync::Mutex};

use actix_web::{
    get, post,
    web::{scope, Data},
    App, HttpResponse, HttpServer, Responder,
};

mod helpers;
mod models;
mod services;

pub mod queries;

use services::{
    create_booking, create_category, create_city, create_course, create_district, create_subcategory,
    get_categories_all, get_cities_by_district, get_courses_all, get_courses_by_id,
    get_district_all, get_subcategories_by_category_id, get_locations_all, get_courses_with_locations
};

pub struct AppState {
    db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = std::env::var("DB_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Error building a connection pool");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState { db: pool.clone() }))
            .service(create_booking)
            .service(create_course)
            .service(create_category)
            .service(create_subcategory)
            .service(create_district)
            .service(create_city)
            .service(get_courses_all)
            .service(get_courses_by_id)
            .service(get_categories_all)
            .service(get_district_all)
            .service(get_cities_by_district)
            .service(get_subcategories_by_category_id)
            .service(get_locations_all)
            .service(get_courses_with_locations)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
