use crate::models::fairings::HeaderValidator;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{ContentType, Method, Status};
use rocket::{Data, Request, Response};

impl Fairing for HeaderValidator {
    fn info(&self) -> Info {
        Info {
            name: "Request Header Validator",
            kind: Kind::Request,
        }
    }

    // fn on_request(&self, request: &mut Request, _: &Data) {
    //
    // }
}

// pub struct RequestTimer;
//
//
// /// Value stored in request-local state.
// #[derive(Copy, Clone)]
// struct TimerStart(Option<SystemTime>);
//
// impl Fairing for RequestTimer {
//     fn info(&self) -> Info {
//         Info {
//             name: "Request Timer",
//             kind: Kind::Request | Kind::Response
//         }
//     }
//
//     /// Stores the start time of the request in request-local state.
//     fn on_request(&self, request: &mut Request, _: &Data) {
//         // Store a `TimerStart` instead of directly storing a `SystemTime`
//         // to ensure that this usage doesn't conflict with anything else
//         // that might store a `SystemTime` in request-local cache.
//         request.local_cache(|| TimerStart(Some(SystemTime::now())));
//     }
//
//     /// Adds a header to the response indicating how long the server took to
//     /// process the request.
//     fn on_response(&self, request: &Request, response: &mut Response) {
//         let start_time = request.local_cache(|| TimerStart(None));
//         if let Some(Ok(duration)) = start_time.0.map(|st| st.elapsed()) {
//             let ms = duration.as_secs() * 1000 + duration.subsec_millis() as u64;
//             response.set_raw_header("X-Response-Time", format!("{} ms", ms));
//         }
//     }
// }
