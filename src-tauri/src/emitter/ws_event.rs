use serde::Serialize;

#[derive(Serialize)]
pub struct WsEvent<'a> {
    pub event: &'a str,
    pub data: &'a dyn erased_serde::Serialize,
}
