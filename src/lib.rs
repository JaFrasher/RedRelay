use red4ext_rs::{export_plugin_symbols, exports, global, wcstr, Exportable, GlobalExport, Plugin, SemVer, U16CStr};

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

fn schedule_request(_request_id: String, _method: String, _url: String, _body: String) {}

fn get_response(_request_id: String) -> String {
    String::from("NOT_FOUND")
}

fn get_status(_request_id: String) -> String {
    String::from("NOT_FOUND")
}

fn cancel_request(_request_id: String) {}
