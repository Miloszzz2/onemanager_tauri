#[derive(Clone, serde::Serialize)]
pub struct Program {
    pub path: String,
    pub visible: bool,
    pub name: String,
    pub favourite: bool,
}
