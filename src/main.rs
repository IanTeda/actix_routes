use actix_web::{
    middleware::{self, Logger},
    web, App, HttpServer,
};

//  Handlers are async functions that receives request-based arguments and return something that can be converted to a response.
pub mod handlers {
    pub mod things {
        use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
        use serde::Deserialize;

        #[derive(Deserialize)]
        struct Info {
            thing_id: String,
        }

        #[get("")]
        async fn index() -> impl Responder {
            HttpResponse::Ok().body("Get one thing.")
        }

        #[get("{thing_id}")]
        async fn get_by_id(info: web::Path<Info>) -> impl Responder {
            let response = format!("Get thing {}.", info.thing_id);
            HttpResponse::Ok().body(response)
        }

        #[post("")]
        async fn create() -> impl Responder {
            HttpResponse::Ok().body("Add a thing.")
        }

        #[put("{thing_id}")]
        async fn update_by_id(info: web::Path<Info>) -> impl Responder {
            let response = format!("Update thing {}.", info.thing_id);
            HttpResponse::Ok().body(response)
        }

        #[delete("{thing_id}")]
        async fn remove_by_id(info: web::Path<Info>) -> impl Responder {
            let response = format!("Delete thing {}.", info.thing_id);
            HttpResponse::Ok().body(response)
        }
    }
}

pub mod routes {
    use actix_web::web;
    use crate::handlers::things;

    pub fn things(config: &mut web::ServiceConfig) {
        config.service(things::index)
            .service(things::get_by_id)
            .service(things::create)
            .service(things::update_by_id)
            .service(things::remove_by_id);
    }
}

pub mod api {
    use crate::routes;
    use actix_web::web;

    pub fn v1(config: &mut web::ServiceConfig) {
        config.service(web::scope("/things").configure(routes::things));
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(middleware::NormalizePath::trim())
            .service(web::scope("/api/v1").configure(api::v1))
        // .service(web::scope("/api/v1")
        //     .service(web::scope("/things")
        //         .service(web::resource("")
        //             .route(web::get().to(get_things))
        //             .route(web::post().to(add_thing))
        //         )
        //     )
        // )
    })
    .bind(("127.0.0.1", 8083))?
    .run()
    .await
}

// References
// https://github.com/actix/examples/tree/master/basics/nested-routing
// https://masteringbackend.com/posts/actix-web-the-ultimate-guide
