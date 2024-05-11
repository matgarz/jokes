use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use rand::Rng;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Joke {
    setup: String,
    punchline: String,
}

fn generate_joke() -> Joke {
    let jokes = vec![
        Joke {
            setup: "Why don't scientists trust atoms?".to_string(),
            punchline: "Because they make up everything!".to_string(),
        },
        Joke {
            setup: "What do you get when you cross a snowman and a vampire?".to_string(),
            punchline: "Frostbite!".to_string(),
        },
    ];

    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..jokes.len());
    jokes[index].clone()
}

async fn joke() -> impl Responder {
    let joke = generate_joke();
    HttpResponse::Ok().json(joke)
}

struct AppState {
    secret_number: i32,
}

async fn init_game() -> AppState {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    AppState { secret_number }
}

async fn guess(state: web::Data<AppState>, num: web::Path<(i32,)>) -> impl Responder {
    let guess = num.0;
    if guess < state.secret_number {
        HttpResponse::Ok().json("Higher")
    } else if guess > state.secret_number {
        HttpResponse::Ok().json("Lower")
    } else {
        HttpResponse::Ok().json("Correct! You guessed it!")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .data_factory(init_game)
            .route("/joke", web::get().to(joke))
            .service(web::resource("/guess/{num}").route(web::get().to(guess)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
