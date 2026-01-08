pub mod adapters;

// use crate::core::adapters::Adapter;
// use flexi_logger::DeferredNow;
// use flexi_logger::{Cleanup, Criterion, FileSpec, Logger, Naming};
// use log;
// use std::io::Write;

// pub fn main(adapter: impl Adapter, projects: Vec<String>, api_key: String) {
//     let _ = configure_logger();
// }

// pub fn configure_logger(
// ) -> std::result::Result<flexi_logger::LoggerHandle, flexi_logger::FlexiLoggerError> {
//     Logger::try_with_env_or_str("info")
//         .unwrap()
//         .format(|w: &mut dyn Write, now: &mut DeferredNow, record| {
//             write!(
//                 w,
//                 "{} [{} | {} | {} | {}] - {}",
//                 now.now().format("%Y-%m-%d %H:%M:%S"),
//                 record.level(),
//                 record.file().unwrap_or("unknown"),
//                 record.module_path().unwrap_or("unknown"),
//                 record.line().unwrap_or(0),
//                 &record.args()
//             )
//         })
//         .log_to_file(FileSpec::default().directory("logs").basename("app"))
//         .rotate(
//             Criterion::Size(2000000),
//             Naming::Timestamps,
//             Cleanup::KeepLogFiles(3),
//         )
//         .start()
// }
