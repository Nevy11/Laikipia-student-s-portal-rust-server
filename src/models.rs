use diesel::prelude::*;

#[derive(Insertable, Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::student_init)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct StudentInit {
    pub reg_no: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub middle_name: String,
    pub year_of_study: i32,
    pub semester: i32,
    pub programme: String,
    pub course: String,
    pub admission_year: i32,
}
/*
reg_no VARCHAR PRIMARY KEY ,
    password VARCHAR NOT NULL,
    first_name VARCHAR NOT NULL,
    middle_name VARCHAR NOT NULL,
    last_name VARCHAR NOT NULL,
    year_of_study INTEGER NOT NULL,
    semester INTEGER NOT NULL,
    programme VARCHAR NOT NULL,
    COURSE VARCHAR NOT NULL,
    admission_year INTEGER NOT NULL
*/

#[derive(Insertable, Selectable, Queryable, Debug)]
#[diesel(table_name=crate::schema::login)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Login {
    pub reg_no: String,
    pub password: String,
}

#[derive(Insertable, Selectable, Queryable, Debug)]
#[diesel(table_name=crate::schema::students_information)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct StudentsInformation {
    pub reg_no: String,
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub year_of_study: i32,
    pub semester: i32,
    pub programme: String,
    pub course: String,
    pub admission_year: i32,
}
/*
reg_no -> Varchar,
    first_name -> Varchar,
    middle_name -> Varchar,
    last_name -> Varchar,
    year_of_study -> Int4,
    semester -> Int4,
    programme -> Varchar,
    course -> Varchar,
    admission_year -> Int4,
 */
