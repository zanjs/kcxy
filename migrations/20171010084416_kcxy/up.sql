CREATE TABLE users (
  id SERIAL NOT NULL PRIMARY KEY,
  email varchar NOT NULL,
  username varchar NOT NULL,
  password varchar NOT NULL,
  created_at timestamp with time zone NOT NULL,
  UNIQUE (email, username)
);