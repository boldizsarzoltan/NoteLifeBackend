CREATE TABLE app_user_refresh (
   id serial PRIMARY KEY,
   user_id int NOT NULL,
   refresh_token VARCHAR (255) NOT NULL,
   application_identifier VARCHAR (255) NOT NULL,
   is_active boolean NULL,
   start_time TIMESTAMP NOT NULL,
   end_time TIMESTAMP NOT NULL
);