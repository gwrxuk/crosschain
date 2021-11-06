#[macro_use] extern crate rocket;
#[get("/")]
fn index() -> &'static str{
	"Cross Chain"
}

#[launch]
fn rocket() -> _{
	rocket::build().mount("/",routes![index])
}
