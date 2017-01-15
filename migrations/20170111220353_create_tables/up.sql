CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  username VARCHAR NOT NULL,
  password VARCHAR NOT NULL
);

CREATE TABLE plugins (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  linux_so VARCHAR NOT NULL,
  windows_dll VARCHAR NOT NULL
);

CREATE TABLE games (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  plugin INTEGER REFERENCES plugins(id)
);

CREATE TABLE servers (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  game INTEGER REFERENCES games(id)
);

CREATE TABLE user_servers (
  user_id INTEGER REFERENCES users(id),
  server_id INTEGER REFERENCES servers(id),
  PRIMARY KEY (user_id, server_id)
);

CREATE TYPE OS AS ENUM ('windows', 'linux');

CREATE TABLE slaves (
  id SERIAL PRIMARY KEY,
  ip VARCHAR NOT NULL,
  port INTEGER NOT NULL DEFAULT 26015,
  os OS
);