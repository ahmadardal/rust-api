CREATE SCHEMA "db";

CREATE TABLE "db"."courses" (
  "id" uuid PRIMARY KEY,
  "course_name" varchar UNIQUE,
  "course_description" varchar,
  "start_date" timestamptz,
  "end_date" timestamptz,
  "csn_entitled" boolean,
  "max_seats" int,
  "image" varchar,
  "days" varchar,
  "hours" varchar,
  "price" int,
  "sessions" int,
  "visible" boolean
);

CREATE TABLE "db"."course_bookings" (
  "id" uuid PRIMARY KEY,
  "course_id" uuid,
  "user_id" uuid,
  "personal_number" bigint,
  "booked_at" timestamptz,
  "paid" boolean
);

CREATE TABLE "db"."locations" (
  "id" uuid PRIMARY KEY,
  "name" varchar UNIQUE,
  "parent_id" uuid,
  "code" int
);

CREATE TABLE "db"."course_location" (
  "course_id" uuid,
  "location_id" uuid,
  PRIMARY KEY ("course_id", "location_id")
);

CREATE TABLE "db"."course_categories" (
  "course_id" uuid,
  "category_id" uuid,
  PRIMARY KEY ("course_id", "category_id")
);

CREATE TABLE "db"."user" (
  "id" uuid PRIMARY KEY,
  "personal_number" bigint,
  "first_name" varchar,
  "last_name" varchar,
  "address" varchar,
  "co" varchar,
  "zipcode" int,
  "city" varchar,
  "kommun" varchar,
  "email" varchar,
  "mobile" varchar
);

CREATE TABLE "db"."categories" (
  "id" uuid PRIMARY KEY,
  "category_name" varchar,
  "parent_id" uuid
);

ALTER TABLE "db"."locations" ADD FOREIGN KEY ("parent_id") REFERENCES "db"."locations" ("id");

ALTER TABLE "db"."categories" ADD FOREIGN KEY ("parent_id") REFERENCES "db"."categories" ("id");

ALTER TABLE "db"."course_categories" ADD FOREIGN KEY ("category_id") REFERENCES "db"."categories" ("id");

ALTER TABLE "db"."course_bookings" ADD FOREIGN KEY ("course_id") REFERENCES "db"."courses" ("id");

ALTER TABLE "db"."course_bookings" ADD FOREIGN KEY ("user_id") REFERENCES "db"."user" ("id");

ALTER TABLE "db"."course_location" ADD FOREIGN KEY ("course_id") REFERENCES "db"."courses" ("id");

ALTER TABLE "db"."course_location" ADD FOREIGN KEY ("location_id") REFERENCES "db"."locations" ("id");

ALTER TABLE "db"."course_categories" ADD FOREIGN KEY ("course_id") REFERENCES "db"."courses" ("id");


-- CREATE VIEW db.full_course_info AS
-- SELECT c.id, c.course_name, c.course_description, c.start_date, c.end_date, c.csn_entitled, c.max_seats, c.image, c.days, c.hours, c.price, c.sessions, c.visible, array_agg(DISTINCT cl.location_id) as city_ids, array_agg(DISTINCT cc.category_id) as subcategory_ids, (SELECT COUNT(*) FROM db.course_bookings WHERE course_id = c.id) as booking_count
-- FROM db.courses c
-- LEFT JOIN db.course_locations cl ON c.id = cl.course_id
-- LEFT JOIN db.course_categories cc ON c.id = cc.course_id
-- LEFT JOIN db.locations l ON cl.location_id = l.id AND l.parent_id IS NOT NULL
-- LEFT JOIN db.categories cat ON cc.category_id = cat.id AND cat.parent_id IS NOT NULL
-- GROUP BY c.id;

CREATE VIEW db.full_course_info AS
SELECT c.id, c.course_name, c.course_description, c.start_date, c.end_date, c.csn_entitled, c.max_seats, c.image, c.days, c.hours, c.price, c.sessions, c.visible, array_agg(DISTINCT l.name) as city_names, array_agg(DISTINCT cat.category_name) as subcategory_names
FROM db.courses c
LEFT JOIN db.course_location cl ON c.id = cl.course_id
LEFT JOIN db.course_categories cc ON c.id = cc.course_id
LEFT JOIN db.locations l ON cl.location_id = l.id AND l.parent_id IS NOT NULL
LEFT JOIN db.categories cat ON cc.category_id = cat.id AND cat.parent_id IS NOT NULL
GROUP BY c.id;

-- CREATE VIEW db.full_course_info AS
-- SELECT c.id, c.course_name, c.course_description, c.start_date, c.end_date, c.csn_entitled, c.max_seats, c.image, c.days, c.hours, c.price, c.sessions, c.visible, array_agg(DISTINCT l.name) as city_names, array_agg(DISTINCT cat.category_name) as subcategory_names
-- FROM db.courses c
-- INNER JOIN db.course_location cl ON c.id = cl.course_id
-- INNER JOIN db.locations l ON cl.location_id = l.id AND l.parent_id IS NOT NULL
-- LEFT JOIN db.course_categories cc ON c.id = cc.course_id
-- LEFT JOIN db.categories cat ON cc.category_id = cat.id AND cat.parent_id IS NOT NULL
-- GROUP BY c.id;


CREATE VIEW db.course_booking_info AS
SELECT  c.id as course_id, c.max_seats, COUNT(cb.id) as booking_count, array_agg(cb.personal_number) as personal_numbers
FROM db.courses c
LEFT JOIN db.course_bookings cb ON c.id = cb.course_id
GROUP BY c.id;

CREATE VIEW db.district_cities AS
SELECT l1.id as district_id, l1.name as district_name, array_agg(l2.id) as cities_id, array_agg(l2.name) as cities_name
FROM db.locations l1
LEFT JOIN db.locations l2 ON l1.id = l2.parent_id
WHERE l1.parent_id IS NULL
GROUP BY l1.id, l1.name;

CREATE VIEW db.category_subcategories AS
SELECT l1.id as category_id, l1.category_name as category_name, array_agg(l2.id) as subcategory_ids, array_agg(l2.category_name) as subcategory_names
FROM db.categories l1
LEFT JOIN db.categories l2 ON l1.id = l2.parent_id
WHERE l1.parent_id IS NULL
GROUP BY l1.id, l1.category_name;