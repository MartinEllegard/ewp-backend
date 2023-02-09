DROP TABLE IF EXISTS "skills_correlation";
DROP TABLE IF EXISTS "skills_users";
DROP TABLE IF EXISTS "skills";
DROP TABLE IF EXISTS "project_users";
DROP TABLE IF EXISTS "projects";
DROP TABLE IF EXISTS "users";
DROP TABLE IF EXISTS "companies";

-- Companies table
CREATE TABLE IF NOT EXISTS "companies" (
    "id" INT GENERATED ALWAYS AS IDENTITY,
    "name" varchar(255) NOT NULL,
    "created_at" timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY ("id")
);

-- Users table
CREATE TABLE IF NOT EXISTS "users" (
    "id" INT GENERATED ALWAYS AS IDENTITY,
    "firstname" varchar(255) NOT NULL,
    "lastname" varchar(255) NOT NULL,
    "description" varchar(255) NOT NULL,
    "email" varchar(255) NOT NULL,
    "company_id" INT,
    "created_at" timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY ("id"),
    CONSTRAINT fk_company
      FOREIGN KEY(company_id) 
	  REFERENCES companies(id)
	  ON DELETE SET NULL
);

-- Oauth access tokens table
CREATE TABLE IF NOT EXISTS "oauth_access_tokens" (
    "id" INT GENERATED always AS IDENTITY,
    "user_id" INT NOT NULL,
    "access_token" varchar(255) NOT NULL,
    "expires_at" timestamp NOT NULL,
    PRIMARY KEY ("id"),
    CONSTRAINT fk_user
        FOREIGN KEY(user_id)
        REFERENCES users(id)
        ON DELETE CASCADE
);

--Create project table
CREATE TABLE IF NOT EXISTS "projects" (
    "id" INT GENERATED ALWAYS AS IDENTITY,
    "name" varchar(255) NOT NULL,
    "description" varchar(255) NOT NULL,
    "created_at" timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY ("id")
);

--Create Skills table
CREATE TABLE IF NOT EXISTS "skills" (
    "id" INT GENERATED ALWAYS AS IDENTITY,
    "name" varchar(255) NOT NULL,
    "use" varchar(255) NOT NULL,
    "created_at" timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY ("id")
);

--Create project skills table
CREATE TABLE IF NOT EXISTS "project_skills" (
    "id" INT GENERATED ALWAYS AS IDENTITY,
    "project_id" INTEGER NOT NULL,
    "skill_id" INTEGER NOT NULL,
    "created_at" timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY ("id"),
    CONSTRAINT fk_project
      FOREIGN KEY(project_id) 
      REFERENCES projects(id),
    Constraint fk_skill
      FOREIGN KEY(skill_id)
      REFERENCES skills(id)
);

--Create Project Users table
CREATE TABLE IF NOT EXISTS "project_users" (
    "id" INT GENERATED ALWAYS AS IDENTITY,
    "project_id" INTEGER NOT NULL,
    "user_id" INTEGER NOT NULL,
    "created_at" timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY ("id"),
    CONSTRAINT fk_project
      FOREIGN KEY(project_id) 
	  REFERENCES projects(id),
    Constraint fk_user
      FOREIGN KEY(user_id)
      REFERENCES users(id)
);

--Create skills users table
CREATE TABLE IF NOT EXISTS "skills_users" (
    "id" INT GENERATED ALWAYS AS IDENTITY,
    "skill_id" INTEGER NOT NULL,
    "user_id" INTEGER NOT NULL,
    "proficiency" INTEGER NOT NULL, -- 1-5
    "created_at" timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY ("id"),
    Constraint fk_skill
      FOREIGN KEY(skill_id)
      REFERENCES skills(id),
    CONSTRAINT fk_user
      FOREIGN KEY(user_id)
      REFERENCES users(id)
);

--Create skill corelation table
CREATE TABLE IF NOT EXISTS "skills_correlation" (
    "id" INT GENERATED ALWAYS AS IDENTITY,
    "skill_primary" INTEGER NOT NULL,
    "skill_secondary" INTEGER NOT NULL,
    "correlation" INTEGER NOT NULL, -- 1-10
    "created_at" timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY ("id"),
    CONSTRAINT fk_skill_primary
      FOREIGN KEY(skill_primary)
      REFERENCES skills(id),
    CONSTRAINT fk_skill_secondary
        FOREIGN KEY(skill_secondary)
        REFERENCES skills(id)
);