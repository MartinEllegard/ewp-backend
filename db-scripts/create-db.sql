--Creating database
CREATE DATABASE db_ewp WITH ENCODING = 'UTF8' CONNECTION LIMIT = -1; 
--\c db_ewp; -- Connect to the database
USE db_ewp; -- Use the database
--CREATE EXTENSION IF NOT EXISTS "uuid-ossp"; -- Enable the uuid extension


--Creating tables

-- Users table
CREATE TABLE IF NOT EXISTS "users" (
    "id" INTEGER PRIMARY KEY AUTOINCREMENT,
    "username" varchar(255) NOT NULL,
    "password" varchar(255) NOT NULL,
    "email" varchar(255) NOT NULL,
    "company" INTEGER FOREIGN KEY REFERENCES "companies"("id"),
    "created_at" timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
);

-- Companies table
CREATE TABLE IF NOT EXISTS "companies" (
    "id" INTEGER PRIMARY KEY AUTOINCREMENT,
    "name" varchar(255) NOT NULL,
    "created_at" timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
);

--Create project table
CREATE TABLE IF NOT EXISTS "projects" (
    "id" INTEGER PRIMARY KEY AUTOINCREMENT,
    "name" varchar(255) NOT NULL,
    "description" varchar(255) NOT NULL,
    "created_at" timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
);

--Create Project Users table
CREATE TABLE IF NOT EXISTS "project_users" (
    "id" INTEGER PRIMARY KEY AUTOINCREMENT,
    "project_id" INTEGER NOT NULL,
    "user_id" INTEGER NOT NULL,
    "created_at" timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY ("project_id") REFERENCES "projects"("id"),
    FOREIGN KEY ("user_id") REFERENCES "users"("id")
);

--Create Skills table
CREATE TABLE IF NOT EXISTS "skills" (
    "id" INTEGER PRIMARY KEY AUTOINCREMENT,
    "name" varchar(255) NOT NULL,
    "use" varchar(255) NOT NULL,
    "created_at" timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
);

--Create skills users table
CREATE TABLE IF NOT EXISTS "skills_users" (
    "id" INTEGER PRIMARY KEY AUTOINCREMENT,
    "skill_id" INTEGER NOT NULL,
    "user_id" INTEGER NOT NULL,
    "proficiency" INTEGER NOT NULL, -- 1-5
    "created_at" timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY ("skill_id") REFERENCES "skills"("id"),
    FOREIGN KEY ("user_id") REFERENCES "users"("id")
);

--Create skill corelation table
CREATE TABLE IF NOT EXISTS "skills_correlation" (
    "id" INTEGER PRIMARY KEY AUTOINCREMENT,
    "skill_id" INTEGER NOT NULL,
    "skill_id2" INTEGER NOT NULL,
    "correlation" INTEGER NOT NULL, -- 0-100
    "created_at" timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY ("skill_id") REFERENCES "skills"("id"),
    FOREIGN KEY ("skill_id2") REFERENCES "skills"("id")
);