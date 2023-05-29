mod test {
    use crate::lib::rocket_builder;
    use rocket::local::Client;
    use rocket::http::Status;

    #[test]
    fn echo_test() {
        let client = Client::new(rocket_builder()).expect("Valid Rocket instance");
        let mut response = client.get("/echo/test_echo").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("test_echo".into()));
    }
}