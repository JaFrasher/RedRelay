use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use red4ext_rs::{export_plugin_symbols, exports, global, wcstr, Exportable, GlobalExport, Plugin, SemVer, U16CStr};

static RESPONSES: OnceLock<Mutex<HashMap<String, String>>> = OnceLock::new();

fn get_map() -> &'static Mutex<HashMap<String, String>> {
    RESPONSES.get_or_init(|| Mutex::new(HashMap::new()))
}

pub struct RedRelay;

impl Plugin for RedRelay {
    const AUTHOR: &'static U16CStr = wcstr!("JaFrasher");
    const NAME: &'static U16CStr = wcstr!("RedRelay");
    const VERSION: SemVer = SemVer::new(0, 1, 0);

    fn exports() -> impl Exportable {
        exports![
            GlobalExport(global!(c"ScheduleRequest", schedule_request)),
            GlobalExport(global!(c"GetResponse", get_response)),
            GlobalExport(global!(c"GetStatus", get_status)),
            GlobalExport(global!(c"CancelRequest", cancel_request)),
        ]
    }
}

export_plugin_symbols!(RedRelay);

fn schedule_request(request_id: String, method: String, url: String, body: String) {
    get_map().lock().unwrap().insert(request_id.clone(), String::from("PENDING"));
    std::thread::spawn(move || {
        let result = match method.to_uppercase().as_str() {
            "GET" => ureq::get(&url).call(),
            _     => ureq::post(&url)
                         .set("Content-Type", "application/json")
                         .send_string(&body),
        };
        let response = match result {
            Ok(resp) => match resp.into_string() {
                Ok(text) => text,
                Err(e)   => format!("ERROR:{}", e),
            },
            Err(e) => format!("ERROR:{}", e),
        };
        get_map().lock().unwrap().insert(request_id, response);
    });
}

fn get_response(request_id: String) -> String {
    match get_map().lock().unwrap().get(&request_id) {
        Some(v) if v == "PENDING" => String::from("PENDING"),
        Some(v)                   => v.clone(),
        None                      => String::from("NOT_FOUND"),
    }
}

fn get_status(request_id: String) -> String {
    match get_map().lock().unwrap().get(&request_id) {
        Some(v) if v == "PENDING"          => String::from("PENDING"),
        Some(v) if v.starts_with("ERROR:") => String::from("ERROR"),
        Some(_)                            => String::from("COMPLETE"),
        None                               => String::from("NOT_FOUND"),
    }
}

fn cancel_request(request_id: String) {
    get_map().lock().unwrap().remove(&request_id);
}
