use rocket;

#[get("/")]
pub fn index() -> &'static str {
    "OWO"
}