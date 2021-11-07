#[macro_use] extern crate rocket;
#[get("/")]
fn index() -> &'static str{
	"Cross Chain"
}

#[get("/world")]
fn world()-> &'static str{
	"hello world"
}
#[launch]
fn rocket() -> _{
	rocket::build().mount("/",routes![index])
}
