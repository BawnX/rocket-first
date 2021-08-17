#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use super::rocket;
use rocket::local::blocking::Client;
use rocket::http::Status;

#[test]
fn test_hello_world() {
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client.get("/api/hello_world").dispatch();
    assert_eq!(response.into_string(), Some("Hola mundo!".into()));
}


#[test]
fn test_greetings() {
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client.get("/api/antonio").dispatch();
    assert_eq!(response.into_string(), Some("Hola antonio".into()));
}

#[test]
fn test_query_greetings_without_salutation() {
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client.get("/api/hello?name=antonio").dispatch();
    assert_eq!(response.into_string(), Some("Hola antonio".into()));
}


#[test]
fn test_query_greetings_with_salutation() {
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client.get("/api/hello?name=antonio&salutation=holiwis").dispatch();
    assert_eq!(response.into_string(), Some("holiwis antonio".into()));
}
