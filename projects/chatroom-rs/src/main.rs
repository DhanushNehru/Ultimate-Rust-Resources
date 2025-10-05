use rocket::{
    fs::{FileServer, relative},
    response::stream::{Event, EventStream},
    tokio::select,
    tokio::sync::{broadcast::{error::RecvError, Sender}, broadcast},
    Shutdown, State,
};
use serde::{Serialize, Deserialize};
use rocket::form::Form;

#[macro_use] extern crate rocket;

#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
struct Message {
    #[field(validate = len(..30))]
    pub room: String,
    #[field(validate = len(..30))]
    pub username: String,
    pub message: String,
}

#[post("/message", data = "<form>")]
fn mailer(form: Form<Message>, queue: &State<Sender<Message>>) {
    let _ = queue.send(form.into_inner());
}

#[get("/events")]
fn get_messages(queue: &State<Sender<Message>>, mut end: Shutdown) -> EventStream![] {
    let mut receiver = queue.subscribe();

    EventStream! {
        loop {
            let message = select! {
                msg = receiver.recv() => match msg {
                    Ok(m) => m,
                    Err(RecvError::Closed) => break,
                    Err(RecvError::Lagged(_)) => continue,
                },
                _ = &mut end => break,
            };

            yield Event::json(&message);
        }
    }
}

#[launch]
fn rocket() -> _ {
    let (tx, _rx) = broadcast::channel::<Message>(1024);

    rocket::build()
        .manage(tx)
        .mount("/", routes![mailer, get_messages])
        .mount("/", FileServer::from(relative!("frontend")))
}
