use super::connection_establish::establish_connection;
use crate::{
    models::{Login, StudentInit, StudentsInformation},
    schema::{login, students_information},
};
use diesel::prelude::*;

pub fn first_years_init(
    reg_no: String,
    password: String,
    first_name: String,
    last_name: String,
    middle_name: String,
    year_of_study: i32,
    semester: i32,
    programme: String,
    course: String,
    admission_year: i32,
) -> String {
    use crate::schema::student_init;
    let connection = &mut establish_connection();
    let data = StudentInit {
        reg_no: reg_no,
        password: password,
        first_name: first_name,
        last_name: last_name,
        middle_name: middle_name,
        year_of_study: year_of_study,
        semester: semester,
        programme: programme,
        course: course,
        admission_year: admission_year,
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
