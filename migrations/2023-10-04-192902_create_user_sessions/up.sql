CREATE TABLE app_user_sessions (
    id serial PRIMARY KEY,
    user_id int NOT NULL,
    access_token VARCHAR (255) NOT NULL,
    application_identifier VARCHAR (255) NOT NULL,
    is_active boolean NULL,
    start_time TIMESTAMP NOT NULL,
    end_time TIMESTAMP NOT NULL,
    UNIQUE (application_identifier, user_id)
);