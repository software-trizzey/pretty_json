use serde_json::{Result, Value};

pub fn prettify_json() -> Result<()> {
    let data: &str = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    let v: Value = serde_json::from_str(data)?;

    println!("Here's {}'s cell #{}", v["name"], v["phones"][0]);

    Ok(())
}
