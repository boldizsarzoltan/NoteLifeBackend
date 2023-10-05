CREATE TABLE app_users (
   id serial PRIMARY KEY,
   user_name VARCHAR (50) NOT NULL,
   email VARCHAR (50) NOT NULL,
   password VARCHAR (255) NOT NULL,
   role VARCHAR (50) NOT NULL,
   UNIQUE (user_name),
   UNIQUE (email)
);