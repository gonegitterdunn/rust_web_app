-------------
----- 1 -----
-------------
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  username VARCHAR NOT NULL,
  password VARCHAR NOT NULL,
  email VARCHAR NOT NULL,
  unique_id VARCHAR NOT NULL,
  UNIQUE (email),
  UNIQUE (username)
);
-------------
----- 2 -----
-------------
INSERT INTO users (username, password, email, unique_id)
VALUES (
    'placeholder',
    'placeholder password',
    'placeholder email',
    'placeholder unique id'
  );
-------------
----- 3 -----
-------------
ALTER TABLE to_do
ADD user_id integer default 1 CONSTRAINT user_id REFERENCES users NOT NULL;