use rocket::{FromForm, post};
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(FromForm, Deserialize)]
struct LoginInfo {
    username: String,
    password: String,
}

#[derive(Debug)]
enum LoginError {
    InvalidData,
    UsernameDoesNotExist,
    WrongPassword
}

// #[post("/register", data="<user>")]
// async fn register_user(username: String, password: String, email: String) ->  {

// }

// pub fn password_checker(password: String) -> bool {
        
//     Argon2::default().verify_password(password.as_bytes(), hash).is_ok()
// }


// impl<'a, 'r> FromRequest<'a, 'r> for AuthenticatedUser {
//     type Error = LoginError;
//     fn from_request(request: &'a Request<'r>) -> Outcome<AuthenticatedUser, LoginError> {
//         let username = request.headers().get_one("username");
//         let password = request.headers().get_one("password");
//         match (username, password) {
//             (Some(u), Some(p)) => {
//                 let conn_str = local_conn_string();
//                 let maybe_user = fetch_user_by_email(&conn_str, &String::from(u));
//                 match maybe_user {
//                     Some(user) => {
//                         let maybe_auth_info = fetch_auth_info_by_user_id(&conn_str, user.id);
//                         match maybe_auth_info {
//                             Some(auth_info) => {
//                                 let hash = hash_password(&String::from(p));
//                                 if hash == auth_info.password_hash {
//                                     Outcome::Success(AuthenticatedUser{user_id: user.id})
//                                 } else {
//                                     Outcome::Failure((Status::Forbidden, LoginError::WrongPassword))
//                                 }
//                             }
//                             None => {
//                                 Outcome::Failure((Status::MovedPermanently, LoginError::WrongPassword))
//                             }
//                         }
//                     }
//                     None => Outcome::Failure((Status::NotFound, LoginError::UsernameDoesNotExist))
//                 }
//             },
//             _ => Outcome::Failure((Status::BadRequest, LoginError::InvalidData))
//         }
//     }
// }