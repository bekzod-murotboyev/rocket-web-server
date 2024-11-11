use std::sync::atomic::{AtomicI32, Ordering};

use rocket::{Data, Orbit, Request, Rocket};
use rocket::fairing::{Fairing, Info, Kind};

pub struct RequestLog {
    pub(crate) count: AtomicI32,
}

#[rocket::async_trait]
impl Fairing for RequestLog {
    fn info(&self) -> Info {
        Info {
            name: "Request Log Fairing",
            kind: Kind::Request | Kind::Shutdown,
        }
    }

    async fn on_request(&self, _req: &mut Request<'_>, _data: &mut Data<'_>) {
        self.count.fetch_add(1, Ordering::Relaxed);
    }

    async fn on_shutdown(&self, _rocket: &Rocket<Orbit>) {
        println!("Requests: {}",self.count.load(Ordering::Relaxed))
    }
}