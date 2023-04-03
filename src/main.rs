use anyhow::Result;
use polars::io::prelude::*;
use std::io::Cursor;

fn main() -> Result<()> {
    let data = serde_json::json!([
        {"a":1, "b":2.0, "c":false, "d":"4"},
        {"a":-10, "b":-3.5, "c":true, "d":"4"},
        {"a":2, "b":0.6, "c":false, "d":"text"},
        {"a":1, "b":2.0, "c":false, "d":"4"},
        {"a":7, "b":-3.5, "c":true, "d":"4"},
        {"a":1, "b":0.6, "c":false, "d":"text"},
        {"a":1, "b":2.0, "c":false, "d":"4"},
        {"a":5, "b":-3.5, "c":true, "d":"4"},
        {"a":1, "b":0.6, "c":false, "d":"text"},
        {"a":1, "b":2.0, "c":false, "d":"4"},
        {"a":1, "b":-3.5, "c":true, "d":"4"},
        {"a":1, "b":0.6, "c":false, "d":"text"},
    ]);
    let cursor = Cursor::new(data.to_string());

    let dfj = JsonReader::new(cursor)
        .with_json_format(JsonFormat::Json)
        .infer_schema_len(Some(3))
        .with_batch_size(3);

    let mut df = dfj.finish()?;

    df = df.sort(["a"], false)?;

    println!("{:?}", df);

    Ok(())
}

//     let basic_json = r#"{"a":1, "b":2.0, "c":false, "d":"4"}
// {"a":-10, "b":-3.5, "c":true, "d":"4"}
// {"a":2, "b":0.6, "c":false, "d":"text"}
// {"a":1, "b":2.0, "c":false, "d":"4"}
// {"a":7, "b":-3.5, "c":true, "d":"4"}
// {"a":1, "b":0.6, "c":false, "d":"text"}
// {"a":1, "b":2.0, "c":false, "d":"4"}
// {"a":5, "b":-3.5, "c":true, "d":"4"}
// {"a":1, "b":0.6, "c":false, "d":"text"}
// {"a":1, "b":2.0, "c":false, "d":"4"}
// {"a":1, "b":-3.5, "c":true, "d":"4"}
// {"a":1, "b":0.6, "c":false, "d":"text"}"#;
