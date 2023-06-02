CREATE TABLE reminders (
   id serial PRIMARY KEY,
   title VARCHAR (50) NOT NULL,
   description TEXT NULL,
   start_time TIMESTAMP NOT NULL,
   end_time TIMESTAMP NOT NULL
);