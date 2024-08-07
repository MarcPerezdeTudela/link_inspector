use serde::Serialize;

#[derive(Serialize)]
pub struct UploadResponse {
    pub error: Option<String>,
    pub links: Vec<CheckedLink>,
}

#[derive(Serialize)]
pub struct CheckedLink {
    pub url: String,
    pub status: Option<u16>,
    pub active: bool,
    pub number: usize,
    pub error: Option<String>,
    pub relocation: Option<String>,
}

pub struct NonCheckedLink {
    pub url: String,
    pub number: usize,
}
#[derive(PartialEq)]
pub enum CheckedFileType {
    Docx,
    Pptx,
    Xlsx,
    Html,
    Invalid,
}
