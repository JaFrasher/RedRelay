use red4ext_rs::prelude::*;

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

fn schedule_request(_request_id: String, _method: String, _url: String, _body: String) {}

fn get_response(_request_id: String) -> String {
    String::from("NOT_FOUND")
}

fn get_status(_request_id: String) -> String {
    String::from("NOT_FOUND")
}

fn cancel_request(_request_id: String) {}
