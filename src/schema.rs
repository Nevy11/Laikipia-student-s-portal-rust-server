// @generated automatically by Diesel CLI.

diesel::table! {
    login (reg_no) {
        reg_no -> Varchar,
        password -> Varchar,
    }
}

diesel::table! {
    student_init (reg_no) {
        reg_no -> Varchar,
        password -> Varchar,
        first_name -> Varchar,
        middle_name -> Varchar,
        last_name -> Varchar,
        year_of_study -> Int4,
        semester -> Int4,
        programme -> Varchar,
        course -> Varchar,
        admission_year -> Int4,
    }
}

diesel::table! {
    students_information (reg_no) {
        reg_no -> Varchar,
        first_name -> Varchar,
        middle_name -> Varchar,
        last_name -> Varchar,
        year_of_study -> Int4,
        semester -> Int4,
        programme -> Varchar,
        course -> Varchar,
        admission_year -> Int4,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    login,
    student_init,
    students_information,
);
