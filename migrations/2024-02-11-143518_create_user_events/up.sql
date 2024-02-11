CREATE TABLE user_events (
   id serial PRIMARY KEY,
   title VARCHAR (50) NOT NULL,
   description TEXT NULL,
   date_time TIMESTAMP NOT NULL,
   event_user_id int NOT NULL,
   FOREIGN KEY(event_user_id) REFERENCES app_users(id)
);