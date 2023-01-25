use crate::{
    models::{
        api::{
            CreateBookingRequest, CreateCategoryRequest, CreateCityRequest, CreateCourseRequest,
            CreateDistrictRequest, CreateSubcategoryRequest,
        },
        db::{
            Category, CategorySubcategories, Course, CourseBookingInfo, DistrictCities, Location,
        },
    },
    AppState,
};
use ::chrono::{DateTime, FixedOffset};
use actix_web::web::{Data, Json};
use sqlx::{self, types::chrono::Utc};
use uuid::Uuid;

pub async fn query_get_course_booking_info(
    state: &Data<AppState>,
    course_id: &Uuid,
) -> Result<CourseBookingInfo, sqlx::Error> {
    let result = sqlx::query_as::<_, CourseBookingInfo>(
        "SELECT * FROM db.course_booking_info WHERE course_id = $1",
    )
    .bind(&course_id)
    .fetch_one(&state.db)
    .await;

    return result;
}

pub async fn query_get_districts_cities_tree(
    state: &Data<AppState>,
) -> Result<Vec<DistrictCities>, sqlx::Error> {
    let districts_cities = sqlx::query_as("SELECT * FROM db.district_cities")
        .fetch_all(&state.db)
        .await;

    return districts_cities;
}

pub async fn query_get_categories_subcategories_tree(
    state: &Data<AppState>,
) -> Result<Vec<CategorySubcategories>, sqlx::Error> {
    let category_subcategories = sqlx::query_as("SELECT * FROM db.category_subcategories")
        .fetch_all(&state.db)
        .await;

    return category_subcategories;
}

pub async fn query_book_course(
    state: &Data<AppState>,
    user_id: &Uuid,
    booking_details: &Json<CreateBookingRequest>,
) -> Result<(), sqlx::Error> {
    let mut tx = state.db.begin().await?;

    let result_create_user = sqlx::query("INSERT INTO db.user (id, personal_number, first_name, last_name, address, co, zipcode, city, kommun, email, mobile) VALUES ($1, $2, $3, $4, $5, NULL, $6, $7, $8, $9, $10)")
            .bind(&user_id)
            .bind(&booking_details.personal_number)
            .bind(&booking_details.first_name)
            .bind(&booking_details.last_name)
            .bind(&booking_details.address)
            .bind(&booking_details.zipcode)
            .bind(&booking_details.city)
            .bind(&booking_details.kommun)
            .bind(&booking_details.email)
            .bind(&booking_details.mobile)
            .execute(&mut tx)
            .await?
            .rows_affected();

    if result_create_user == 0 {
        return tx.rollback().await;
    }

    let booking_id = Uuid::new_v4();
    let booked_at = Utc::now();

    let result_create_booking = sqlx::query("INSERT INTO db.course_bookings (id, course_id, user_id, personal_number, booked_at, paid) VALUES ($1, $2, $3, $4, $5, False)")
            .bind(&booking_id)
            .bind(&booking_details.course_id)
            .bind(&user_id)
            .bind(&booking_details.personal_number)
            .bind(&booked_at)
            .execute(&mut tx)
            .await?
            .rows_affected();

    if result_create_booking == 0 {
        return tx.rollback().await;
    }

    return tx.commit().await;
}

pub async fn query_add_course(
    state: &Data<AppState>,
    id: &Uuid,
    start_date: &DateTime<FixedOffset>,
    end_date: &DateTime<FixedOffset>,
    course_details: &Json<CreateCourseRequest>,
) -> Result<(), sqlx::Error> {
    let mut tx = state.db.begin().await?;

    let result_create_course = sqlx::query("INSERT INTO db.courses (id, course_name, course_description, start_date, end_date, csn_entitled, max_seats, image, days, hours, price, sessions, visible) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13 )")
            .bind(&id)
            .bind(&course_details.course_name.to_string())
            .bind(&course_details.course_description.to_string())
            .bind(&start_date)
            .bind(&end_date)
            .bind(&course_details.csn_entitled)
            .bind(&course_details.max_seats)
            .bind(&course_details.image.to_string())
            .bind(&course_details.days.to_string())
            .bind(&course_details.hours.to_string())
            .bind(&course_details.price)
            .bind(&course_details.sessions)
            .bind(&course_details.visible)
            .execute(&mut tx)
            .await?
            .rows_affected();

    if result_create_course == 0 {
        return tx.rollback().await;
    }

    if !&course_details.city_ids.is_empty() {
        let mut cities_added: u64 = 0;
        for city_id in &course_details.city_ids {
            let result = sqlx::query(
                "INSERT INTO db.course_location (course_id, location_id) VALUES ($1, $2)",
            )
            .bind(id)
            .bind(city_id)
            .execute(&mut tx)
            .await?
            .rows_affected();
            cities_added += result
        }

        if cities_added == 0 {
            return tx.rollback().await;
        }
    }

    if !&course_details.subcategory_ids.is_empty() {
        let mut subcategories_added: u64 = 0;

        for subcategory_id in &course_details.subcategory_ids {
            let result = sqlx::query(
                "INSERT INTO db.course_categories (course_id, category_id) VALUES ($1, $2)",
            )
            .bind(id)
            .bind(subcategory_id)
            .execute(&mut tx)
            .await?
            .rows_affected();
            subcategories_added += result
        }

        if subcategories_added == 0 {
            return tx.rollback().await;
        }
    }

    let result = tx.commit().await;

    return result;
}

pub async fn query_get_cities_by_district(
    state: &Data<AppState>,
    parent_id: &Uuid,
) -> Result<Vec<Location>, sqlx::Error> {
    let result = sqlx::query_as::<_, Location>("SELECT * FROM db.locations WHERE parent_id = $1")
        .bind(&parent_id)
        .fetch_all(&state.db)
        .await;

    return result;
}

pub async fn query_get_city_by_name(
    state: &Data<AppState>,
    parent_id: &Uuid,
    name: &String,
) -> Result<Location, sqlx::Error> {
    let result = sqlx::query_as::<_, Location>(
        "SELECT * FROM db.locations WHERE name = $1 AND parent_id = $2",
    )
    .bind(&name)
    .bind(&parent_id)
    .fetch_one(&state.db)
    .await;

    return result;
}

pub async fn query_get_subcategory_by_name(
    state: &Data<AppState>,
    parent_id: &Uuid,
    name: &String,
) -> Result<Category, sqlx::Error> {
    let result = sqlx::query_as::<_, Category>(
        "SELECT * FROM db.categories WHERE category_name = $1 AND parent_id = $2",
    )
    .bind(&name)
    .bind(&parent_id)
    .fetch_one(&state.db)
    .await;

    return result;
}

pub async fn query_get_category_by_name(
    state: &Data<AppState>,
    name: &String,
) -> Result<Category, sqlx::Error> {
    let result = sqlx::query_as::<_, Category>(
        "SELECT * FROM db.categories WHERE category_name = $1 AND parent_id IS NULL",
    )
    .bind(name)
    .fetch_one(&state.db)
    .await;

    return result;
}

pub async fn query_get_category_by_id(
    state: &Data<AppState>,
    category_id: &Uuid,
) -> Result<Category, sqlx::Error> {
    let result = sqlx::query_as::<_, Category>(
        "SELECT * FROM db.categories WHERE id = $1 AND parent_id IS NULL",
    )
    .bind(category_id)
    .fetch_one(&state.db)
    .await;

    return result;
}

pub async fn query_get_categories_all(
    state: &Data<AppState>,
) -> Result<Vec<Category>, sqlx::Error> {
    let result =
        sqlx::query_as::<_, Category>("SELECT * FROM db.categories WHERE parent_id IS NULL")
            .fetch_all(&state.db)
            .await;

    return result;
}

pub async fn query_get_subcategories_by_categoryid(
    state: &Data<AppState>,
    category_id: &Uuid,
) -> Result<Vec<Category>, sqlx::Error> {
    let result = sqlx::query_as::<_, Category>("SELECT * FROM db.categories WHERE parent_id = $1")
        .bind(category_id)
        .fetch_all(&state.db)
        .await;

    return result;
}

pub async fn query_create_category(
    state: Data<AppState>,
    id: &Uuid,
    body: &Json<CreateCategoryRequest>,
) -> Result<Category, sqlx::Error> {
    let result = sqlx::query_as::<_, Category>(
        "INSERT INTO db.categories (id, category_name, parent_id) VALUES ($1, $2, NULL) RETURNING id, category_name, parent_id",
    )
    .bind(id)
    .bind(body.category_name.to_string())
    .fetch_one(&state.db)
    .await;
    return result;
}

pub async fn query_create_subcategory(
    state: Data<AppState>,
    id: &Uuid,
    parent_id: &Uuid,
    body: &Json<CreateSubcategoryRequest>,
) -> Result<Category, sqlx::Error> {
    let result = sqlx::query_as::<_, Category>(
        "INSERT INTO db.categories (id, category_name, parent_id) VALUES ($1, $2, $3) RETURNING id, category_name, parent_id"
    )
        .bind(id)
        .bind(body.category_name.to_string())
        .bind(parent_id)
        .fetch_one(&state.db)
        .await;
    return result;
}

pub async fn query_courses(state: Data<AppState>, id: &Uuid) -> Result<Vec<Course>, sqlx::Error> {
    let result = sqlx::query_as::<_, Course>("SELECT * FROM db.courses")
        .bind(id)
        .fetch_all(&state.db)
        .await;

    return result;
}

pub async fn query_get_course_by_id(
    state: &Data<AppState>,
    id: &Uuid,
) -> Result<Course, sqlx::Error> {
    let result = sqlx::query_as::<_, Course>("SELECT * FROM db.courses WHERE id = $1")
        .bind(id)
        .fetch_one(&state.db)
        .await;

    return result;
}

pub async fn query_get_courses_by_category(
    state: &Data<AppState>,
    category_name: String,
) -> Result<Vec<Course>, sqlx::Error> {
    let result = sqlx::query_as::<_, Course>("SELECT * FROM db.courses")
        .fetch_all(&state.db)
        .await;

    return result;
}

pub async fn query_get_course_by_name(
    state: &Data<AppState>,
    name: String,
) -> Result<Course, sqlx::Error> {
    let result = sqlx::query_as::<_, Course>("SELECT * FROM db.courses WHERE course_name = $1")
        .bind(name)
        .fetch_one(&state.db)
        .await;

    return result;
}

pub async fn query_get_all_courses(state: &Data<AppState>) -> Result<Vec<Course>, sqlx::Error> {
    let result = sqlx::query_as::<_, Course>("SELECT * FROM db.full_course_info")
        .fetch_all(&state.db)
        .await;

    return result;
}

pub async fn query_get_location_by_city(
    state: &Data<AppState>,
    city: String,
) -> Result<Location, sqlx::Error> {
    let result = sqlx::query_as::<_, Location>("SELECT * FROM db.locations WHERE city = $1")
        .bind(city)
        .fetch_one(&state.db)
        .await;

    return result;
}

pub async fn query_get_district_by_id(
    state: &Data<AppState>,
    id: &Uuid,
) -> Result<Location, sqlx::Error> {
    let result = sqlx::query_as::<_, Location>(
        "SELECT * FROM db.locations WHERE id = $1 AND parent_id IS NULL",
    )
    .bind(&id)
    .fetch_one(&state.db)
    .await;

    return result;
}

pub async fn query_get_districts(state: &Data<AppState>) -> Result<Vec<Location>, sqlx::Error> {
    let result =
        sqlx::query_as::<_, Location>("SELECT * FROM db.locations WHERE parent_id IS NULL")
            .fetch_all(&state.db)
            .await;

    return result;
}

pub async fn query_create_district(
    state: &Data<AppState>,
    id: Uuid,
    location_details: &Json<CreateDistrictRequest>,
) -> Result<Location, sqlx::Error> {
    let result = sqlx::query_as::<_, Location>(
        "INSERT INTO db.locations (id, name, parent_id, code) VALUES ($1, $2, NULL, $3) RETURNING id, name, parent_id, code",
    )
    .bind(id)
    .bind(location_details.name.to_string())
    .bind(location_details.code)
    .fetch_one(&state.db)
    .await;

    return result;
}

pub async fn query_create_city(
    state: &Data<AppState>,
    id: &Uuid,
    parent_id: &Uuid,
    location_details: &Json<CreateCityRequest>,
) -> Result<Location, sqlx::Error> {
    let result = sqlx::query_as::<_, Location>(
        "INSERT INTO db.locations (id, name, parent_id, code) VALUES ($1, $2, $3, $4) RETURNING id, name, parent_id, code",
    )
    .bind(&id)
    .bind(&location_details.name)
    .bind(&parent_id)
    .bind(&location_details.code)
    .fetch_one(&state.db)
    .await;

    return result;
}
