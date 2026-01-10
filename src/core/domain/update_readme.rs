use crate::core::adapters::{Adapter, FileType, IoValue};
use crate::core::domain::errors::PepyStatsError;
use crate::core::domain::transform::ReadMeTable;
use regex::Regex;
use std::path::PathBuf;

#[derive(Debug, Eq, PartialEq)]
pub struct ReadMe(String);

pub fn update_readme(
    adapter: &mut impl Adapter,
    readme_table: ReadMeTable,
    readme_path: &str,
) -> Result<(), PepyStatsError> {
    let readme_path = PathBuf::from(readme_path);
    log::info!("{:?}", readme_table);
    log::info!("{:?}", readme_path);

    let current_readme = ReadMe(adapter.read(&readme_path, FileType::Str)?.to_string()?);
    let updated_readme = parse_readme_table(&current_readme, readme_table)?;
    log::info!("{:?}", current_readme);
    log::info!("{:?}", updated_readme);
    if updated_readme != current_readme {
        adapter.write(&readme_path, IoValue::Str(updated_readme.0), FileType::Str)?;
        log::info!("updated readme");
    }
    Ok(())
}

fn parse_readme_table(
    readme: &ReadMe,
    readme_table: ReadMeTable,
) -> Result<ReadMe, PepyStatsError> {
    let pattern = Regex::new(r"(?s)(## python packages)(.*?)(::)")?;
    Ok(ReadMe(
        pattern
            .replace(&readme.0, format!("$1\n{}\n$3", readme_table.into_string()))
            .to_string(),
    ))
}
