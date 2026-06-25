-- Add migration script here
-- Roles of the users 
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE roles (
  id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
  role TEXT NOT NULL
);

INSERT INTO roles (role) VALUES
  ('admin'),
  ('user');

-- Table to store the users info
CREATE TABLE users (
  id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
  username VARCHAR(16) NOT NULL,
  role_id uuid NOT NULL,
  password VARCHAR(128) NOT NULL,
  subscribed_at TIMESTAMPTZ,  

  CONSTRAINT fk_role FOREIGN KEY (role_id)
  REFERENCES roles(id)       
);
