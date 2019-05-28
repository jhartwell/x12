use toml::de::Error;
use toml::map::Map;
use toml::value::Table;
use toml::Value;

pub struct Config {}

pub struct Loop {
    path: String,
    length: usize,
    elements: Vec<String>,
}

fn loops(loop_table: &Table) -> Result<Loop, Error> {
    Ok(Loop {
        path: "test".to_owned(),
        length: 10,
        elements: vec![],
    })
}

pub fn load(config: &str) -> Result<Config, Error> {
    let a = config.parse::<Value>()?;
    let b = a["loop"].as_table();
    Ok(Config {})
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn run() {
        let toml = r#"
version = "5010x218"
format = "820"
[[loop]]
    [loop.1000A]
        name = "Premium Receiver's Name"
        n1 = [""]
        n2 = [""]
        n3 = [""]
        n4 = [""]
        rdm = [""]
    [loop.1000B]
        name = "Premium Payer's Name"
        n1 = [""]
        n2 = [""]
        n3 = [""]
        n4 = [""]
        per = [""]
    [loop.1000C]
        name = "Intermediary Bank Information"
    [loop.2000A]
        name = "Organization Summary Remittance"
        [loop.2000A.2200A]
            name = "Organization Summary Remittance Level Adjustment For Previous Payment"
        [loop.2000A.2300A]
            name = "Organization Summary Remittance Detail"
            [loop.2000A.2300A.2310A]
                name = "Summary Line Item"
            [loop.2000A.2300A.2312A]
                name = "Service, Promotion, Allowance, Or Charge Information"
            [loop.2000A.2300A.2315A]
                name = "Member Count"

        "#;

        let v = toml.parse::<Value>().unwrap();
        if let Some(loops) = v["loop"].as_array() {
            for k in loops {
                if let Some(table) = k.as_table() {
                    for (t, v) in table {
                        println!("{:?} -> {:?}", t, v);
                    }
                }
            }
        }
        assert!(false);
    }
}
