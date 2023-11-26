use std::fmt;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index).service(name))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(r##"
            <head>
                <script src="https://unpkg.com/htmx.org@1.9.9" integrity="sha384-QFjmbokDn2DjBjq+fM+8LUIVrAgqcNW2s0PjAxHETgRn9l4fvX31ZxDxvwQnyMOX" crossorigin="anonymous"></script>
            </head>
            <body>
                <h1>Hello to the HTMX x Rust Demo</h1>

                <form hx-post="/name" hx-target="#name">
                    <input type="text" name="name" placeholder="What's your name?">
                    <button type="submit">Submit</button>
                </form>

                <div id="name"></div>
            </body>
        "##)
}

#[derive(Deserialize, Debug)]
struct Form {
    name: String,
}

impl fmt::Display for Form {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[post("/name")]
async fn name(form: web::Form<Form>) -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(format!("form is: {}", form))
}
