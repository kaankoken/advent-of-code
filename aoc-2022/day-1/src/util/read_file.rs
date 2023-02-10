use color_eyre::eyre::Context;

pub fn read_file(file_name: String) -> color_eyre::Result<String> {
    let file =
        std::fs::read_to_string(file_name.clone()).wrap_err(format!("Reading {file_name}"))?;

    Ok(file)
}
