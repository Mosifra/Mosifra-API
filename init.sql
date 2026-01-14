-- Table type de formation
CREATE TABLE course_type (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL
);

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
    course_type INT REFERENCES course_type(id) ON DELETE CASCADE,
    start_date DATE,
    end_date DATE,
    min_length INT, -- Minimum stage length in weeks
    max_length INT, -- Maximum stage length in weeks
    university_id VARCHAR(128) REFERENCES university(id) ON DELETE CASCADE
);


-- Table étudiant
CREATE TABLE student (
    id VARCHAR(128) PRIMARY KEY,
    first_name VARCHAR(100) NOT NULL,
    last_name VARCHAR(100) NOT NULL,
    login VARCHAR(100) UNIQUE NOT NULL,
    password VARCHAR(255) NOT NULL,
    mail VARCHAR(255) UNIQUE NOT NULL,
    class_id VARCHAR(128) REFERENCES class(id) ON DELETE CASCADE
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
    university_id VARCHAR(128) REFERENCES university(id) ON DELETE SET NULL,
    start_date DATE, -- Total start time
    end_date DATE, -- Total end time
    min_internship_length INT, -- Minimum length of the internship
    max_internship_length INT, -- Maximum length of the internship
    title VARCHAR(255),
    description TEXT,
    place VARCHAR(255)
    
    CONSTRAINT chek_internship_creator CHECK (
        (company_id IS NOT NULL AND university_id IS NULL)
     OR (company_id IS NULL AND university_id IS NOT NULL)
    )
);

-- Relation université <-> stage
CREATE TABLE university_internship (
    university_id VARCHAR(128) REFERENCES university(id) ON DELETE CASCADE,
    internship_id VARCHAR(128) REFERENCES internship(id) ON DELETE CASCADE,
    PRIMARY KEY (university_id, internship_id)
);
INSERT INTO course_type (name) VALUES ('info'); -- 1
