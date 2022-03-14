-------------
----- 1 -----
-------------
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  username VARCHAR NOT NULL,
  email VARCHAR NOT NULL,
  password VARCHAR NOT NULL,
  uuid VARCHAR NOT NULL,
  UNIQUE (email),
  UNIQUE (username)
);
-------------
----- 2 -----
-------------
INSERT INTO users (username, email, password, uuid)
VALUES (
    'placeholder',
    'placeholder email',
    'placeholder password',
    'placeholder unique id'
  );
-------------
----- 3 -----
-------------
ALTER TABLE to_do
ADD user_id integer default 1 CONSTRAINT user_id REFERENCES users NOT NULL;