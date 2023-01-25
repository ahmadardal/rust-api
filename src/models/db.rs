use serde::{Deserialize, Serialize};
use sqlx::types::chrono::DateTime;
use sqlx::{
    self,
    types::chrono::Utc,
};
use uuid::Uuid;

#[derive(sqlx::FromRow, Serialize)]
pub struct Course {
    pub id: Uuid,
    pub course_name: String,
    pub course_description: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub csn_entitled: bool,
    pub max_seats: i32,
    pub image: String,
    pub days: String,
    pub hours: String,
    pub price: i32,
    pub sessions: i32,
    pub visible: bool,
    pub city_names: Vec<Option<String>>,
    pub subcategory_names: Vec<Option<String>>
    // pub subcategory_ids: Vec<Uuid>,
    // pub booking_count: i64,
}

#[derive(sqlx::FromRow, Serialize)]
pub struct CourseBookingInfo {
    pub course_id: Uuid,
    pub max_seats: i32,
    pub booking_count: i64,
    pub personal_numbers: Vec<Option<i64>>,
}

#[derive(sqlx::FromRow)]
pub struct User {
    id: Uuid,
    personal_number: i64,
    first_name: String,
    last_name: String,
    address: String,
    co: String,
    zipcode: String,
    city: String,
    kommun: String,
    email: String,
    mobile: String,
}

#[derive(sqlx::FromRow)]
pub struct CourseBooking {
    id: Uuid,
    course_id: i32,
    user_id: i32,
    booked_at: DateTime<Utc>,
    paid: bool,
}

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct Location {
    id: Uuid,
    name: String,
    parent_id: Option<Uuid>,
    code: i32,
}

#[derive(sqlx::FromRow)]
pub struct CourseLocation {
    id: Uuid,
    course_id: i32,
    location_id: i32,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct Category {
    id: Uuid,
    category_name: String,
    parent_id: Option<Uuid>,
}

#[derive(sqlx::FromRow, Serialize)]
pub struct DistrictCities {
    pub district_id: Uuid,
    pub district_name: String,
    pub cities_id: Vec<Option<Uuid>>,
    pub cities_name: Vec<Option<String>>,
}

#[derive(sqlx::FromRow, Serialize)]
pub struct CategorySubcategories {
    pub category_id: Uuid,
    pub category_name: String,
    pub subcategory_ids: Vec<Option<Uuid>>,
    pub subcategory_names: Vec<Option<String>>,
}

#[derive(Serialize)]
pub struct Subcategory {
    pub subcategory_id: Option<Uuid>,
    pub subcategory_name: Option<String>,
}
#[derive(Serialize)]
pub struct NestedCategory {
    pub category_id: Uuid,
    pub category_name: String,
    pub subcategories: Vec<Option<Subcategory>>
}

#[derive(Serialize)]
pub struct City {
    pub city_id: Option<Uuid>,
    pub city_name: Option<String>,
}

#[derive(Serialize)]
pub struct District {
    pub district_id: Uuid,
    pub district_name: String,
    pub cities: Vec<Option<City>>,
}

#[derive(Serialize)]
pub struct CoursesCategoriesDistricts {
    pub courses: Vec<Course>,
    pub categories: Vec<NestedCategory>,
    pub districts: Vec<District>,
}
