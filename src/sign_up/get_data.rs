use diesel::prelude::*;

use crate::{
    models::{Login, StudentsInformation},
    sign_up::connection_establish::establish_connection,
};

pub fn get_data() -> String {
    use crate::schema::{
        login::dsl::login as ldsl, students_information::dsl::students_information as sdls,
    };

    let connection = &mut establish_connection();
    let data = Login {
        reg_no: String::from("SC/COM/0041/22"),
        password: String::from("Skyworth.95"),
    };
    let login_details = ldsl
        .find(&data.reg_no)
        .select(Login::as_select())
        .first(connection)
        .optional();

    println!("{:?}", login_details);
    match login_details {
        Ok(Some(details)) => {
            if data.reg_no == details.reg_no && data.password == details.password {
                let user_details = sdls
                    .find(data.reg_no.clone())
                    .select(StudentsInformation::as_returning())
                    .first(connection)
                    .optional();
                println!("user login data: {:?}", user_details);
                match user_details {
                    Ok(Some(data)) => format!("{:?}", data),
                    Ok(None) => format!("None"),
                    Err(_) => format!("Error getting the user's information"),
                }
            } else {
                format!("Loggin unsuccessfull")
            }
        }
        Ok(None) => {
            format!("The student's info is not found")
        }
        Err(_) => format!("An error occured"),
    }
}
