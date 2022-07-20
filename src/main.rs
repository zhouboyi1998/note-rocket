use rocket::{
    catch, catchers, delete, get, launch, post, put, routes,
    serde::json::{serde_json::json, Value},
};

#[get("/")]
async fn list() -> Value {
    json!("list")
}

#[get("/<id>")]
async fn one(id: i32) -> Value {
    json!(format!("one {}", id))
}

#[post("/")]
async fn insert() -> Value {
    json!("insert")
}

#[put("/")]
async fn update() -> Value {
    json!("update")
}

#[delete("/<id>")]
async fn delete(id: i32) -> Value {
    json!(format!("delete {}", id))
}

#[catch(404)]
async fn not_found() -> Value {
    json!("URL Not Found!")
}

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .mount("/product", routes![list, one, insert, update, delete])
        .register("/", catchers!(not_found))
}
