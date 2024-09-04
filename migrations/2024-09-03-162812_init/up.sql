-- Your SQL goes here
CREATE TABLE student_init (
    reg_no VARCHAR PRIMARY KEY ,
    password VARCHAR NOT NULL,
    first_name VARCHAR NOT NULL,
    middle_name VARCHAR NOT NULL,
    last_name VARCHAR,
    year_of_study INTEGER NOT NULL,
    semester INTEGER NOT NULL,
    programme VARCHAR NOT NULL,
    COURSE VARCHAR NOT NULL,
    admission_year VARCHAR NOT NULL
)