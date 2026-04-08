use std::collections::HashMap;
use std::sync::Mutex;
use red4ext_rs::prelude::*;

static RESPONSES: Mutex<Option<HashMap<String, String>>> = Mutex::new(None);

fn init_storage() {
    let mut guard = RESPONSES.lock().unwrap();
    if guard.is_none() {
        *guard = Some(HashMap::new());
    }
}

define_plugin! {
    name: "RedRelay",
    author: "JaFrasher",
    version: 0:1:0,
    on_register: {
        register_function!("ScheduleRequest", schedule_request);
        register_function!("GetResponse", get_response);
        register_function!("GetStatus", get_status);
        register_function!("CancelRequest", cancel_request);
    }
}

fn schedule_request(request_id: String, method: String, url: String, body: String) {
    init_storage();
    {
        let mut guard = RESPONSES.lock().unwrap();
        let map = guard.as_mut().unwrap();
        map.insert(request_id.clone(), String::from("PENDING"));
    }
    std::thread::spawn(move || {
        let result = match method.to_uppercase().as_str() {
            "GET" => ureq::get(&url).call(),
            _ => ureq::post(&url)
                .set("Content-Type", "application/json")
                .send_string(&body),
        };
        let response = match result {
            Ok(resp) => match resp.into_string() {
                Ok(text) => text,
                Err(e) => format!("ERROR:{}", e),
            },
            Err(e) => format!("ERROR:{}", e),
        };
        let mut guard = RESPONSES.lock().unwrap();
        let map = guard.as_mut().unwrap();
        map.insert(request_id, response);
    });
}

fn get_response(request_id: String) -> String {
    init_storage();
    let guard = RESPONSES.lock().unwrap();
    let map = guard.as_ref().unwrap();
    match map.get(&request_id) {
        Some(v) if v == "PENDING" => String::from("PENDING"),
        Some(v) => v.clone(),
        None => String::from("NOT_FOUND"),
    }
}

fn get_status(request_id: String) -> String {
    init_storage();
    let guard = RESPONSES.lock().unwrap();
    let map = guard.as_ref().unwrap();
    match map.get(&request_id) {
        Some(v) if v == "PENDING" => String::from("PENDING"),
        Some(v) if v.starts_with("ERROR:") => String::from("ERROR"),
        Some(_) => String::from("COMPLETE"),
        None => String::from("NOT_FOUND"),
    }
}

fn cancel_request(request_id: String) {
    init_storage();
    let mut guard = RESPONSES.lock().unwrap();
    let map = guard.as_mut().unwrap();
    map.remove(&request_id);
}
