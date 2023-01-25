use crate::{
    models::{
        api::{
            CreateBookingRequest, CreateCategoryRequest, CreateCityRequest, CreateCourseRequest,
            CreateDistrictRequest,
        },
        db::{City, CoursesCategoriesDistricts, District, NestedCategory, Subcategory},
    },
    queries::{
        query_add_course, query_book_course, query_create_category, query_create_city,
        query_create_district, query_create_subcategory, query_get_all_courses,
        query_get_categories_subcategories_tree, query_get_category_by_id,
        query_get_category_by_name, query_get_cities_by_district, query_get_city_by_name,
        query_get_course_booking_info, query_get_course_by_id, query_get_course_by_name,
        query_get_district_by_id, query_get_districts, query_get_districts_cities_tree,
        query_get_subcategories_by_categoryid, query_get_subcategory_by_name,
    },
    AppState,
};

use actix_web::{
    get, post,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};

use sqlx::{self, types::chrono::DateTime};
use uuid::Uuid;

use super::models::api::CreateSubcategoryRequest;

#[get("/courses")]
pub async fn get_courses_all(state: Data<AppState>) -> impl Responder {
    match query_get_all_courses(&state).await {
        Ok(courses) => HttpResponse::Ok().json(courses),
        Err(_) => HttpResponse::NotFound().json("No courses found"),
    }
}

#[get("/coursesWithCategoriesAndLocations")]
pub async fn get_courses_with_locations(state: Data<AppState>) -> impl Responder {
    let courses = match query_get_all_courses(&state).await {
        Ok(courses) => courses,
        Err(_) => return HttpResponse::NotFound().json("No courses found"),
    };

    let categories = fetch_categories_and_subcategories(&state).await;

    let districts = fetch_districts_and_cities(&state).await;

    let response = CoursesCategoriesDistricts {
        courses,
        categories,
        districts,
    };

    return HttpResponse::Accepted().json(response);
}

pub async fn fetch_categories_and_subcategories(state: &Data<AppState>) -> Vec<NestedCategory> {
    let mut response: Vec<NestedCategory> = vec![];

    let category_subcategories = match query_get_categories_subcategories_tree(&state).await {
        Ok(category_subcategories) => category_subcategories,
        Err(_) => return response,
    };

    for category in category_subcategories {
        let zipped: Vec<(Option<Uuid>, Option<String>)> = category
            .subcategory_ids
            .iter()
            .zip(category.subcategory_names.iter())
            .map(|(id, name)| (id.clone(), name.clone()))
            .collect();

        let mut subcategories: Vec<Option<Subcategory>> = vec![];

        for element in &zipped {
            if !element.0.is_none() {
                let subcategory = Subcategory {
                    subcategory_id: element.0.clone(),
                    subcategory_name: element.1.clone(),
                };

                subcategories.push(Some(subcategory));
            }
        }

        let nested_category = NestedCategory {
            category_id: category.category_id,
            category_name: category.category_name,
            subcategories,
        };

        response.push(nested_category);
    }
    return response;
}

pub async fn fetch_districts_and_cities(state: &Data<AppState>) -> Vec<District> {
    let mut response: Vec<District> = vec![];

    let district_cities = match query_get_districts_cities_tree(&state).await {
        Ok(district_cities) => district_cities,
        Err(err) => return response,
    };

    for district in district_cities {
        let zipped: Vec<(Option<Uuid>, Option<String>)> = district
            .cities_id
            .iter()
            .zip(district.cities_name.iter())
            .map(|(id, name)| (id.clone(), name.clone()))
            .collect();

        let mut cities: Vec<Option<City>> = vec![];

        for element in &zipped {
            if !element.0.is_none() {
                let city = City {
                    city_id: element.0.clone(),
                    city_name: element.1.clone(),
                };

                cities.push(Some(city));
            }
        }

        let distr = District {
            district_id: district.district_id,
            district_name: district.district_name,
            cities,
        };

        response.push(distr);
    }
    return response;
}

#[get("/locations")]
pub async fn get_locations_all(state: Data<AppState>) -> impl Responder {
    let locations: Vec<District> = fetch_districts_and_cities(&state).await;
    return HttpResponse::Ok().json(locations);
}

#[get("/subcategories/{id}")]
pub async fn get_subcategories_by_category_id(
    state: Data<AppState>,
    path: Path<String>,
) -> impl Responder {
    let parent_id: Uuid = match uuid::Uuid::try_parse(&path.into_inner()) {
        Ok(id) => id,
        Err(err) => {
            return HttpResponse::BadRequest().json("Could not parse category_id as a UUID!")
        }
    };

    match query_get_subcategories_by_categoryid(&state, &parent_id).await {
        Ok(subcategories) => HttpResponse::Ok().json(subcategories),
        Err(_) => HttpResponse::NotFound().json("No subcategories found"),
    }
}

#[post("/course")]
pub async fn create_course(
    state: Data<AppState>,
    body: Json<CreateCourseRequest>,
) -> impl Responder {
    // Check if course with this name already exists
    let course_exists = query_get_course_by_name(&state, body.course_name.to_string()).await;

    if course_exists.is_ok() {
        return HttpResponse::BadRequest().json("Course with this name already exists!");
    }

    // Otherwise create new id, convert dates from string to datetime and query the db
    let id = Uuid::new_v4();

    let start_date = match DateTime::parse_from_rfc3339(&body.start_date) {
        Ok(parsed_date) => parsed_date,
        Err(err) => {
            // handle the error
            println!("An error occurred while parsing the date: {:?}", err);
            return HttpResponse::BadRequest().json("Could not parse start_date!");
        }
    };

    let end_date = match DateTime::parse_from_rfc3339(&body.end_date) {
        Ok(parsed_date) => parsed_date,
        Err(err) => {
            // handle the error
            println!("An error occurred while parsing the date: {:?}", err);
            return HttpResponse::BadRequest().json("Could not parse end_date!");
        }
    };

    match query_add_course(&state, &id, &start_date, &end_date, &body).await {
        Ok(_) => HttpResponse::Ok().json("Course added!"),
        Err(err) => {
            if err.to_string().contains("duplicate") {
                return HttpResponse::BadRequest().json("Course already exists!");
            } else {
                return HttpResponse::InternalServerError().json(err.to_string());
            }
        }
    }
}

#[post("/district")]
pub async fn create_district(
    state: Data<AppState>,
    body: Json<CreateDistrictRequest>,
) -> impl Responder {
    let id = Uuid::new_v4();

    match query_create_district(&state, id, &body).await {
        Ok(location) => HttpResponse::Ok().json(location),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

#[post("/city")]
pub async fn create_city(state: Data<AppState>, body: Json<CreateCityRequest>) -> impl Responder {
    // Try to parse the district_id as a UUID
    let parent_id = match uuid::Uuid::try_parse(&body.district_id) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().json("Could not parse parent_id as a UUID!"),
    };

    // Check that the provided district actually exists
    let district_exists = query_get_district_by_id(&state, &parent_id).await;

    if !district_exists.is_ok() {
        return HttpResponse::BadRequest().json("Parent district does not exist!");
    }

    // Check if city already exists under the same district
    let city_exists = query_get_city_by_name(&state, &parent_id, &body.name).await;

    if city_exists.is_ok() {
        return HttpResponse::BadRequest()
            .json("City with this name already exists in this district!");
    }

    // Create the city
    let id = Uuid::new_v4();

    match query_create_city(&state, &id, &parent_id, &body).await {
        Ok(location) => HttpResponse::Ok().json(location),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

// Add a category
#[post("/category")]
pub async fn create_category(
    state: Data<AppState>,
    body: Json<CreateCategoryRequest>,
) -> impl Responder {
    let category_exists = query_get_category_by_name(&state, &body.category_name).await;

    if category_exists.is_ok() {
        return HttpResponse::BadRequest().json("Category with this name already exists!");
    }

    let id = Uuid::new_v4();
    match query_create_category(state, &id, &body).await {
        Ok(category) => HttpResponse::Ok().json(category),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

// Add a subcategory
#[post("/subcategory")]
pub async fn create_subcategory(
    state: Data<AppState>,
    body: Json<CreateSubcategoryRequest>,
) -> impl Responder {
    // Check if category exists

    let parent_id = match uuid::Uuid::try_parse(&body.parent_id) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().json("Could not parse parent_id as a UUID!"),
    };

    let category_exists = query_get_category_by_id(&state, &parent_id).await;

    if !category_exists.is_ok() {
        return HttpResponse::BadRequest().json("Parent category does not exist!");
    }

    // Check if subcategory already exists

    let subcategory_exists =
        query_get_subcategory_by_name(&state, &parent_id, &body.category_name).await;

    if subcategory_exists.is_ok() {
        return HttpResponse::BadRequest().json("Subcategory with this name already exists!");
    }

    // Create subcategory

    let id = Uuid::new_v4();
    match query_create_subcategory(state, &id, &parent_id, &body).await {
        Ok(subcategory) => HttpResponse::Ok().json(subcategory),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

#[get("/coursesBySubcategoryId/{id}")]
pub async fn get_courses_by_subcategory_id(
    state: Data<AppState>,
    path: Path<String>,
) -> impl Responder {
    let id: String = path.into_inner();

    // match query_get_courses(&state, id).await {
    //     Ok(course) => HttpResponse::Ok().json(course),
    //     Err(_) => HttpResponse::NotFound().json("No course with given id found!"),
    // }

    return HttpResponse::Ok().json("Hello");
}

#[get("/courses/{id}")]
pub async fn get_courses_by_id(state: Data<AppState>, path: Path<String>) -> impl Responder {
    let id: Uuid = match uuid::Uuid::try_parse(&path.into_inner()) {
        Ok(id) => id,
        Err(err) => return HttpResponse::BadRequest().json("Could not parse parent_id as a UUID!"),
    };

    match query_get_course_by_id(&state, &id).await {
        Ok(course) => HttpResponse::Ok().json(course),
        Err(_) => HttpResponse::NotFound().json("No course with given id found!"),
    }
}

#[get("/cities/{id}")]
pub async fn get_cities_by_district(state: Data<AppState>, path: Path<String>) -> impl Responder {
    let parent_id: Uuid = match uuid::Uuid::try_parse(&path.into_inner()) {
        Ok(id) => id,
        Err(err) => return HttpResponse::BadRequest().json("Could not parse parent_id as a UUID!"),
    };

    match query_get_cities_by_district(&state, &parent_id).await {
        Ok(course) => HttpResponse::Ok().json(course),
        Err(_) => HttpResponse::NotFound().json("No cities with given parent_id found!"),
    }
}

#[get("/categories")]
pub async fn get_categories_all(state: Data<AppState>) -> impl Responder {
    // match query_get_categories_all(&state).await {
    //     Ok(categories) => HttpResponse::Ok().json(categories),
    //     Err(_) => HttpResponse::NotFound().json("Error fetching categories!"),
    // }

    let categories = fetch_categories_and_subcategories(&state).await;

    return HttpResponse::Ok().json(categories);
}

#[get("/districts")]
pub async fn get_district_all(state: Data<AppState>) -> impl Responder {
    match query_get_districts(&state).await {
        Ok(districts) => HttpResponse::Ok().json(districts),
        Err(_) => HttpResponse::NotFound().json("Error fetching districts!"),
    }
}

#[post("/booking")]
pub async fn create_booking(
    state: Data<AppState>,
    body: Json<CreateBookingRequest>,
) -> impl Responder {
    // Check if there are free seats remaining in Course
    let course_booking_info = match query_get_course_booking_info(&state, &body.course_id).await {
        Ok(course_booking_info) => course_booking_info,
        Err(err) => return HttpResponse::BadRequest().json(err.to_string()),
    };

    if course_booking_info.booking_count >= course_booking_info.max_seats as i64 {
        return HttpResponse::Conflict().json("The course is fully booked!");
    }

    // Check if the user already has booked this course earlier
    let personal_numbers = course_booking_info
        .personal_numbers
        .into_iter()
        .filter_map(|x| x)
        .collect::<Vec<i64>>();

    if personal_numbers.contains(&body.personal_number) {
        return HttpResponse::Conflict().json("You have already booked this course!");
    }

    // Add the new user to the database, and create the booking.
    let user_id = Uuid::new_v4();

    match query_book_course(&state, &user_id, &body).await {
        Ok(_) => return HttpResponse::Created().json("Booking made!"),
        Err(err) => {
            println!("Hello");
            return HttpResponse::BadRequest().json(err.to_string());
        }
    }
}
