use std::fs::File;
use std::io::{self, BufReader};
use std::path::Path;
use serde::{Deserialize, Serialize};
use tauri::command;

#[derive(Debug, Deserialize, Serialize)]
pub struct Record {
    name: String,
    #[serde(rename = "IP")]
    ip: String,
    delay:String
}
// 定义 RecordList 结构
#[derive(Debug, Serialize, Deserialize)]
pub struct RecordList {
    list: Vec<Record>,
}
// 从 JSON 文件中读取 RecordList 信息
fn read_records_from_json<P: AsRef<Path>>(path: P) -> io::Result<RecordList> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let records: RecordList = serde_json::from_reader(reader)?;
    Ok(records)
}

// 定义 CmdParams 结构
#[derive(Deserialize)]
pub struct CmdParams {
    path: String,
}

// 定义一个 Tauri 命令
#[command]
pub fn get_records(params: CmdParams) -> Result<RecordList, tauri::Error> {
    read_records_from_json(params.path).map_err(|e| e.into())
}