#[cfg(test)]
mod test {
    use rocket::local::Client;
    use rocket::http::Status;
    
    use crate::rocket_builder;
    
    #[test]
    fn echo_test() {
        let client = Client::new(rocket_builder()).expect("Valid Rocket instance");
        let mut response = client.get("/echo/test_echo").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("test_echo".into()));
    }
}