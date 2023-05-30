#[cfg(test)]
mod test {
    use rocket::local::blocking::Client;
    use rocket::http::Status;
    
    use crate::rocket_builder;
    
    #[test]
    fn echo_test() {
        let client = Client::tracked(rocket_builder()).expect("valid `Rocket`");
        let response = client.get("/echo/test_echo").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), "test_echo");
    }
}