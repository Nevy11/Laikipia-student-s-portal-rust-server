use super::connection_establish::establish_connection;
use crate::{
    models::{Login, StudentInit, StudentsInformation},
    schema::{login, students_information},
};
use diesel::prelude::*;

pub fn first_years_init() -> String {
    use crate::schema::student_init;
    let connection = &mut establish_connection();
    let data = StudentInit {
        reg_no: String::from("SC/COM/0041/22"),
        password: String::from("Skyworth.95"),
        first_name: String::from("Stephen"),
        last_name: String::from("Mongare"),
        middle_name: String::from("Mainda"),
        year_of_study: 3,
        semester: 1,
        programme: String::from("BACHELOR OF SCIENCE"),
        course: String::from("COMPUTER SCIENCE"),
        admission_year: 2022,
    };
    let login_addition = Login {
        reg_no: data.reg_no.clone(),
        password: data.password.clone(),
    };
    let student_info = StudentsInformation {
        reg_no: login_addition.reg_no.clone(),
        first_name: data.first_name.clone(),
        middle_name: data.middle_name.clone(),
        last_name: data.last_name.clone(),
        year_of_study: data.year_of_study.clone(),
        semester: data.semester.clone(),
        programme: data.programme.clone(),
        course: data.course.clone(),
        admission_year: data.admission_year.clone(),
    };
    //student's information
    diesel::insert_into(students_information::table)
        .values(student_info)
        .returning(StudentsInformation::as_returning())
        .get_result(connection)
        .expect("Error adding the students information to the database");
    // login
    diesel::insert_into(login::table)
        .values(&login_addition)
        .returning(Login::as_returning())
        .get_result(connection)
        .expect("Error adding the new data to login database");
    // student's init
    diesel::insert_into(student_init::table)
        .values(&data)
        .returning(StudentInit::as_returning())
        .get_result(connection)
        .expect("Error saving new data");
    format!("The user has been updated successfully")
}
