use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::db::{Location, Category};

#[derive(Deserialize)]
pub struct CreateCourseRequest {
    pub course_name: String,
    pub course_description: String,
    pub start_date: String,
    pub end_date: String,
    pub csn_entitled: bool,
    pub max_seats: i32,
    pub image: String,
    pub days: String,
    pub hours: String,
    pub price: i32,
    pub sessions: i32,
    pub visible: bool,
    pub city_ids: Vec<Uuid>,
    pub subcategory_ids: Vec<Uuid>,
}

#[derive(Deserialize)]
pub struct CreateDistrictRequest {
    pub name: String,
    pub code: i32,
}

#[derive(Deserialize)]
pub struct CreateCityRequest {
    pub name: String,
    pub district_id: String,
    pub code: i32,
}

#[derive(Deserialize)]
pub struct CreateSubcategoryRequest {
    pub category_name: String,
    pub parent_id: String,
}

#[derive(Deserialize)]
pub struct CreateCategoryRequest {
    pub category_name: String,
}



#[derive(Deserialize)]
pub struct CreateBookingRequest {
    pub personal_number: i64,
    pub first_name: String,
    pub last_name: String,
    pub address: String,
    pub zipcode: i32,
    pub city: String,
    pub kommun: String,
    pub email: String,
    pub mobile: String,
    pub course_id: Uuid,
}