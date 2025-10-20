-- Table université
CREATE TABLE university (
    id VARCHAR(128) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    login VARCHAR(100) NOT NULL,
    password VARCHAR(255) NOT NULL,
    mail VARCHAR(255) UNIQUE NOT NULL
);

-- Table promo
CREATE TABLE class (
    id VARCHAR(128) PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    course_type INT REFERENCES course_type(id) ON DELETE RESTRICT,
    start_date DATE,
    end_date DATE,
    min_size INT,
    max_size INT,
    university_id VARCHAR(128) REFERENCES university(id) ON DELETE RESTRICT
);


-- Table étudiant
CREATE TABLE student (
    id VARCHAR(128) PRIMARY KEY,
    first_name VARCHAR(100) NOT NULL,
    last_name VARCHAR(100) NOT NULL,
    login VARCHAR(100) UNIQUE NOT NULL,
    password VARCHAR(255) NOT NULL,
    mail VARCHAR(255) UNIQUE NOT NULL,
    class_id VARCHAR(128) REFERENCES class(id) ON DELETE RESTRICT
);

-- Table type de formation
CREATE TABLE course_type (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL
);

-- Table entreprise
CREATE TABLE company (
    id VARCHAR(128) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    login VARCHAR(100) UNIQUE NOT NULL,
    password VARCHAR(255) NOT NULL,
    mail VARCHAR(255) UNIQUE NOT NULL
);

-- Table stage
CREATE TABLE internship (
    id VARCHAR(128) PRIMARY KEY,
    course_type INT REFERENCES course_type(id) ON DELETE RESTRICT,
    company_id VARCHAR(128) REFERENCES company(id) ON DELETE SET NULL,
    start_date DATE,
    end_date DATE,
    min_internship_length INT,
    max_internship_length INT,
    title VARCHAR(255),
    description TEXT,
    place VARCHAR(255)
);

-- Relation université <-> stage
CREATE TABLE university_internship (
    university_id VARCHAR(128) REFERENCES university(id) ON DELETE RESTRICT,
    internship_id VARCHAR(128) REFERENCES internship(id) ON DELETE RESTRICT,
    PRIMARY KEY (university_id, internship_id)
);
