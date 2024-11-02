// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms

#![deny(warnings, clippy::all)]

use log::info;

mod journal;

use similar_asserts::assert_eq;
use systemd_journal_logger::{CustomFieldsProvider, JournalLog};

struct FieldsProvider {}

impl CustomFieldsProvider for FieldsProvider {
    fn get(&self) -> Box<dyn Iterator<Item = (String, Vec<u8>)>> {
        let kvs: Vec<(String, Vec<u8>)> = vec![
            (
                "THREAD_ID".into(),
                format!("{:?}", std::thread::current().id()).into(),
            ),
            ("CUSTOM_FIELD".into(), "custom".into()),
        ];
        Box::new(kvs.into_iter())
    }
}

#[test]
fn init_with_custom_fields() {
    JournalLog::new()
        .unwrap()
        .with_custom_fields_provider(FieldsProvider {})
        .install()
        .unwrap();
    log::set_max_level(log::LevelFilter::Info);

    info!(target: "init_with_extra_fields", "Hello World");

    let entry = journal::read_one_entry("init_with_extra_fields");
    assert_eq!(entry["TARGET"], "init_with_extra_fields");
    assert_eq!(entry["MESSAGE"], "Hello World");
    assert!(entry.contains_key("THREAD_ID"));
    assert_eq!(entry["CUSTOM_FIELD"], "custom");
}
