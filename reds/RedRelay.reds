native func ScheduleRequest(request_id: String, method: String, url: String, body: String);
native func GetResponse(request_id: String) -> String;
native func GetStatus(request_id: String) -> String;
native func CancelRequest(request_id: String);
