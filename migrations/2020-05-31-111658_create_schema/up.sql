CREATE TABLE foods (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL
);


CREATE TABLE meal_types (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL
);

CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL UNIQUE,
  password VARCHAR NOT NULL
);

CREATE TABLE entries (
  id SERIAL PRIMARY KEY,
  user_id INTEGER REFERENCES users (id) NOT NULL,
  meal_type_id INTEGER REFERENCES meal_types (id) NOT NULL,
  timestamp TIMESTAMP NOT NULL,
  comments VARCHAR
);

CREATE TABLE meals (
  CONSTRAINT id PRIMARY KEY (food_id, entry_id),
  food_id INTEGER REFERENCES foods (id) ON UPDATE CASCADE ON DELETE CASCADE,
  entry_id INTEGER REFERENCES entries (id) ON UPDATE CASCADE ON DELETE CASCADE
);