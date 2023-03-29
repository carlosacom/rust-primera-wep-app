#[macro_use] extern crate rocket;

use rocket::serde::json::Json;
use serde::Serialize;

#[derive(Serialize)]
struct Person {
    name: String,
    age: i32,
}

#[get("/")]
fn index() -> &'static str {
    "¡Bienvenido a la API de libros!"
}

#[get("/libros/<id>")]
fn libro(id: usize) -> String {
    format!("Aquí se muestra el libro con ID {}.", id)
}

#[get("/")]
fn people() -> Json<Person> {
    // Construye el objeto JSON
    let person = Person {
        name: "John".to_string(),
        age: 30,
    };
    // Retorna el objeto JSON en el response
    Json(person)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/api", routes![libro])
        .mount("/personas", routes![people])
}
