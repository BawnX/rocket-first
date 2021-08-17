// #![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
// #![feature(proc_macro_hygiene, decl_macro)]
//
#[macro_use]
extern crate rocket;

use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::RwLock;

use reqwest;
use rocket::http::{CookieJar, Cookie};
use rocket::response::status::Created;
use rocket::{State, Data, Rocket, Build};
use rocket::fairing::{Info, Kind, Fairing};
use rocket::request::Request;
use rocket::{serde::json::Json, serde::json::Value};
use serde::{Serialize, Deserialize};
use serde_json::json;

// #[cfg(test)]
// mod test;

mod api_key;


#[get("/hello_world")]
fn hello_world() -> &'static str {
    "Hola mundo!"
}

#[get("/<name>")]
fn greeting_name(name: String) -> String {
    format!("Hola {}", name)
}

#[get("/hello?<name>&<salutation>")]
fn query_name(name: String, salutation: Option<String>) -> String {
    match salutation {
        Some(s) => format!("{} {}", s, name),
        None => format!("Hola {}", name)
    }
}

#[get("/protected")]
fn protected(key: api_key::ApiKey) -> String {
    format!("Tienes permiso para entrar con la key {}", key.0)
}

#[get("/login")]
fn login(cookies: &CookieJar ) {
    cookies.add(Cookie::new("Session", base64::encode("Esta_es_una_nueva_galleta")))
}

#[get("/session")]
fn session(cookies: &CookieJar) -> &'static str {
    match cookies.get("Session") {
        Some(_) => "Tu tienes la galleta",
        None => "Lo siento, no tengo la galleta"
    }
}

type ID = usize;

#[derive(Serialize, Debug, Clone)]
struct Heroe {
    id: ID,
    name: String,
    #[serde(rename(serialize = "canFly"))]
    can_fly: bool,
}

#[derive(Deserialize, Debug)]
struct NewHero {
    name: String,
    #[serde(rename(deserialize = "canFly"))]
    can_fly: bool,
}

struct HeroesCount(AtomicUsize);

type HeroesMap = RwLock<HashMap<ID, Heroe>>;

#[post("/heroes", format = "json", data = "<heroe>")]
fn add_heroe(
    heroe: Json<NewHero>,
    heroes_state: &State<HeroesMap>,
    heroes_count: &State<HeroesCount>,
) -> Created<Json<Heroe>> {
    let hid = heroes_count.0.fetch_add(1, Ordering::Relaxed);
    let new_heroe = Heroe {
        id: hid,
        name: heroe.0.name,
        can_fly: heroe.0.can_fly,
    };

    let mut heroes = heroes_state.write().unwrap();
    heroes.insert(hid, new_heroe.clone());

    let location = uri!(get_heroe(hid));
    Created::new(location.to_string()).body(Json(new_heroe))
}

#[get("/heroes/<id>")]
fn get_heroe(id: ID, heroes_state: &State<HeroesMap>) -> Option<Json<Heroe>> {
    let heroes = heroes_state.read().unwrap();
    heroes.get(&id).map(|h| Json(h.clone()))
}

#[get("/heroes")]
fn get_all_heroes(heroes_state: &State<HeroesMap>) -> Json<Vec<Heroe>> {
    let heroes = heroes_state.read().unwrap();
    Json(heroes.values().map(|v| v.clone()).collect())
}

#[catch(404)]
fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Recurso no encontrado"
    })
}

#[derive(Serialize, Deserialize, Debug)]
struct LogEvent {
    #[serde(rename(serialize = "@t"))]
    timestamp: chrono::DateTime<chrono::Utc>,

    #[serde(rename(serialize = "@mt"))]
    message_tempate: &'static str,

    #[serde(rename(serialize = "path"))]
    path: String,
}

struct LogTarget;

#[rocket::async_trait]
impl Fairing for LogTarget {
    fn info(&self) -> Info {
        Info {
            name: "Log to Seq",
            kind: Kind::Request,
        }
    }

    async fn on_request(&self, req: &mut Request<'_>, _: &mut Data<'_>) {
        let event = LogEvent {
            timestamp: chrono::Utc::now(),
            message_tempate: "Request al {path}",
            path: req.uri().path().to_owned().to_string(),
        };

        reqwest::Client::new()
            .post("http://seq:5341/api/events/raw?clef")
            .json(&event)
            .send()
            .await
            .unwrap();
    }
}

fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount(
            "/api",
            routes![
                hello_world,
                greeting_name,
                query_name,
                protected,
                login,
                session,
                add_heroe,
                get_heroe,
                get_all_heroes
            ],
        )
        .manage(RwLock::new(HashMap::<ID, Heroe>::new()))
        .manage(HeroesCount(AtomicUsize::new(1)))
        .register("/api", catchers![not_found])
        .attach(LogTarget{})
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error>{
    rocket().launch().await
}
