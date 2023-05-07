use std::io::{self, BufRead, Cursor, Write};

use prost::Message;

use crate::gen::CodeGenResponse;

use super::gen::{CodeGenRequest, Table};

pub fn get_tables(req: &CodeGenRequest) -> Vec<Table> {
    req.catalog
        .iter()
        .flat_map(|c| c.schemas.iter())
        .flat_map(|s| s.tables.iter())
        .map(|t| t.clone())
        .collect::<Vec<Table>>()
}

pub fn deserialize_codegen_request(buf: &[u8]) -> Result<CodeGenRequest, prost::DecodeError> {
    CodeGenRequest::decode(&mut Cursor::new(buf))
}

pub fn serialize_codegen_response(resp: &CodeGenResponse) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.reserve(resp.encoded_len());

    resp.encode(&mut buf).unwrap();
    buf
}

pub fn run_plugin(
    build_res: fn(&CodeGenRequest) -> CodeGenResponse,
) -> Result<(), prost::DecodeError> {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let buffer = stdin.fill_buf().unwrap();

    let req = match deserialize_codegen_request(&buffer) {
        Ok(request_deserialized_result) => request_deserialized_result,
        Err(_e) => std::process::exit(1),
    };

    let resp = build_res(&req);
    let out = serialize_codegen_response(&resp);

    let _ = match io::stdout().write_all(&out) {
        Ok(result) => result,
        Err(_e) => std::process::exit(1),
    };

    Ok(())
}
