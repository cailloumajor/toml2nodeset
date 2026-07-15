use anyhow::Context;
use schemars::schema_for;
use toml2nodeset::ObjectType;

fn main() -> anyhow::Result<()> {
    let schema = schema_for!(ObjectType);
    let json = serde_json::to_string_pretty(&schema).context("Failed to serialize JSON schema")?;

    println!("{json}");

    Ok(())
}
