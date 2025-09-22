-- Table étudiant
CREATE TABLE student (
    id SERIAL PRIMARY KEY,
    first_name VARCHAR(100) NOT NULL,
    last_name VARCHAR(100) NOT NULL,
    login VARCHAR(100) UNIQUE NOT NULL,
    password VARCHAR(255) NOT NULL,
    mail VARCHAR(255) UNIQUE NOT NULL,
    twofa VARCHAR(255) NULL
);

-- Table type de formation
CREATE TABLE course_type (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL
);

-- Table promo
CREATE TABLE class (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    course_type INT REFERENCES course_type(id) ON DELETE RESTRICT,
    start_date DATE,
    end_date DATE,
    min_size INT,
    max_size INT
);

-- Relation n-n promo <-> étudiant
CREATE TABLE student_class (
    class_id INT REFERENCES class(id) ON DELETE RESTRICT,
    student_id INT REFERENCES student(id) ON DELETE RESTRICT,
    PRIMARY KEY (class_id, student_id)
);

-- Table université
CREATE TABLE university (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    login VARCHAR(100) NOT NULL,
    password VARCHAR(255) NOT NULL,
    mail VARCHAR(255) UNIQUE NOT NULL,
    twofa VARCHAR(255) NULL
);

-- Relation université <-> promo
CREATE TABLE university_class (
    university_id INT REFERENCES university(id) ON DELETE RESTRICT,
    class_id INT REFERENCES class(id) ON DELETE RESTRICT,
    PRIMARY KEY (university_id, class_id)
);

-- Table entreprise
CREATE TABLE company (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    login VARCHAR(100) UNIQUE NOT NULL,
    password VARCHAR(255) NOT NULL,
    mail VARCHAR(255) UNIQUE NOT NULL,
    twofa VARCHAR(255) NULL
);

-- Table stage
CREATE TABLE internship (
    id SERIAL PRIMARY KEY,
    course_type INT REFERENCES course_type(id) ON DELETE RESTRICT,
    company_id INT REFERENCES company(id) ON DELETE SET NULL,
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
    university_id INT REFERENCES university(id) ON DELETE RESTRICT,
    internship_id INT REFERENCES internship(id) ON DELETE RESTRICT,
    PRIMARY KEY (university_id, internship_id)
);
