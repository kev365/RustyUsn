extern crate rusty_usn;
extern crate serde_json;
use std::io::Cursor;
use rusty_usn::record;


#[test]
fn usn_record_json_value_test() {
    let v3_record_buffer: &[u8] = &[
        0x70,0x00,0x00,0x00,0x03,0x00,0x00,0x00,0xB9,0x8A,0x00,0x00,0x00,0x00,0x02,0x00,
        0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xC8,0x07,0x00,0x00,0x00,0x00,0x02,0x00,
        0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x60,0x78,0xA2,0x9A,0x01,0x00,0x00,0x00,
        0xE9,0xB6,0x4E,0x4D,0xE0,0x65,0xD5,0x01,0x02,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
        0x00,0x00,0x00,0x00,0x20,0x00,0x00,0x00,0x20,0x00,0x4C,0x00,0x43,0x00,0x49,0x00,
        0x44,0x00,0x6F,0x00,0x77,0x00,0x6E,0x00,0x6C,0x00,0x6F,0x00,0x61,0x00,0x64,0x00,
        0x65,0x00,0x72,0x00,0x2E,0x00,0x6C,0x00,0x6F,0x00,0x67,0x00,0x00,0x00,0x00,0x00
    ];

    let v3_record = match record::UsnRecord::new(3, v3_record_buffer) {
        Ok(record) => record,
        Err(error) => {
            eprintln!("{:?}", error);
            panic!(error);
        }
    };

    let v3_json_value = v3_record.to_json_value(None).unwrap();
    assert_eq!(&v3_json_value.to_string(), r#"{"file_attributes":"FILE_ATTRIBUTE_ARCHIVE","file_name":"CIDownloader.log","file_name_length":32,"file_name_offset":76,"file_reference":{"entry":35513,"sequence":2,"u128":"562949953456825"},"major_version":3,"minor_version":0,"parent_reference":{"entry":1992,"sequence":2,"u128":"562949953423304"},"reason":"USN_REASON_DATA_EXTEND","record_length":112,"security_id":0,"source_info":"(empty)","timestamp":"2019-09-08T00:56:52.138160Z","usn":6889306208}"#);

    let record_meta = record::EntryMeta::new(
        "Test Buffer", 
        0
    );

    let v3_json_value_additional = v3_record.to_json_value(
        Some(record_meta.to_json_value().unwrap())
    ).unwrap();
    
    assert_eq!(&v3_json_value_additional.to_string(), r#"{"file_attributes":"FILE_ATTRIBUTE_ARCHIVE","file_name":"CIDownloader.log","file_name_length":32,"file_name_offset":76,"file_reference":{"entry":35513,"sequence":2,"u128":"562949953456825"},"major_version":3,"meta__offset":0,"meta__source":"Test Buffer","minor_version":0,"parent_reference":{"entry":1992,"sequence":2,"u128":"562949953423304"},"reason":"USN_REASON_DATA_EXTEND","record_length":112,"security_id":0,"source_info":"(empty)","timestamp":"2019-09-08T00:56:52.138160Z","usn":6889306208}"#);
}

#[test]
fn usn_records_json_test() {
    let v3_record_buffer: &[u8] = &[
        0x70,0x00,0x00,0x00,0x03,0x00,0x00,0x00,0xB9,0x8A,0x00,0x00,0x00,0x00,0x02,0x00,
        0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xC8,0x07,0x00,0x00,0x00,0x00,0x02,0x00,
        0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x60,0x78,0xA2,0x9A,0x01,0x00,0x00,0x00,
        0xE9,0xB6,0x4E,0x4D,0xE0,0x65,0xD5,0x01,0x02,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
        0x00,0x00,0x00,0x00,0x20,0x00,0x00,0x00,0x20,0x00,0x4C,0x00,0x43,0x00,0x49,0x00,
        0x44,0x00,0x6F,0x00,0x77,0x00,0x6E,0x00,0x6C,0x00,0x6F,0x00,0x61,0x00,0x64,0x00,
        0x65,0x00,0x72,0x00,0x2E,0x00,0x6C,0x00,0x6F,0x00,0x67,0x00,0x00,0x00,0x00,0x00
    ];

    let v3_record = match record::UsnRecord::new(3, v3_record_buffer) {
        Ok(record) => record,
        Err(error) => panic!(error)
    };

    let v3_json_str = serde_json::to_string(&v3_record).unwrap();
    assert_eq!(v3_json_str, r#"{"record_length":112,"major_version":3,"minor_version":0,"file_reference":{"u128":"562949953456825","entry":35513,"sequence":2},"parent_reference":{"u128":"562949953423304","entry":1992,"sequence":2},"usn":6889306208,"timestamp":"2019-09-08T00:56:52.138160Z","reason":"USN_REASON_DATA_EXTEND","source_info":"(empty)","security_id":0,"file_attributes":"FILE_ATTRIBUTE_ARCHIVE","file_name_length":32,"file_name_offset":76,"file_name":"CIDownloader.log"}"#);


    // Record v2 Test
    let v2_record_buffer: &[u8] = &[
        0x60,0x00,0x00,0x00,0x02,0x00,0x00,0x00,0x73,0x00,0x00,0x00,0x00,0x00,0x68,0x91,
        0x3B,0x2A,0x02,0x00,0x00,0x00,0x07,0x00,0x00,0x00,0x80,0xBC,0x04,0x00,0x00,0x00,
        0x53,0xC7,0x8B,0x18,0xC5,0xCC,0xCE,0x01,0x02,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
        0x00,0x00,0x00,0x00,0x20,0x20,0x00,0x00,0x20,0x00,0x3C,0x00,0x42,0x00,0x54,0x00,
        0x44,0x00,0x65,0x00,0x76,0x00,0x4D,0x00,0x61,0x00,0x6E,0x00,0x61,0x00,0x67,0x00,
        0x65,0x00,0x72,0x00,0x2E,0x00,0x6C,0x00,0x6F,0x00,0x67,0x00,0x00,0x00,0x00,0x00
    ];

    let v2_record = match record::UsnRecord::new(2, v2_record_buffer) {
        Ok(record) => record,
        Err(error) => panic!(error)
    };
    let v2_json_str = serde_json::to_string(
        &v2_record
    ).unwrap();

    assert_eq!(v2_json_str, r#"{"record_length":96,"major_version":2,"minor_version":0,"file_reference":{"entry":115,"sequence":37224},"parent_reference":{"entry":141883,"sequence":7},"usn":20342374400,"timestamp":"2013-10-19T12:16:53.276040Z","reason":"USN_REASON_DATA_EXTEND","source_info":"(empty)","security_id":0,"file_attributes":"FILE_ATTRIBUTE_ARCHIVE | FILE_ATTRIBUTE_NOT_CONTENT_INDEXED","file_name_length":32,"file_name_offset":60,"file_name":"BTDevManager.log"}"#);
}

#[test]
fn usn_record_v3_test() {
    let record_buffer: &[u8] = &[
        0x70,0x00,0x00,0x00,0x03,0x00,0x00,0x00,0xB9,0x8A,0x00,0x00,0x00,0x00,0x02,0x00,
        0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xC8,0x07,0x00,0x00,0x00,0x00,0x02,0x00,
        0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x60,0x78,0xA2,0x9A,0x01,0x00,0x00,0x00,
        0xE9,0xB6,0x4E,0x4D,0xE0,0x65,0xD5,0x01,0x02,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
        0x00,0x00,0x00,0x00,0x20,0x00,0x00,0x00,0x20,0x00,0x4C,0x00,0x43,0x00,0x49,0x00,
        0x44,0x00,0x6F,0x00,0x77,0x00,0x6E,0x00,0x6C,0x00,0x6F,0x00,0x61,0x00,0x64,0x00,
        0x65,0x00,0x72,0x00,0x2E,0x00,0x6C,0x00,0x6F,0x00,0x67,0x00,0x00,0x00,0x00,0x00
    ];

    let record = match record::UsnRecordV3::new(&mut Cursor::new(record_buffer)) {
        Ok(record) => record,
        Err(error) => panic!(error)
    };

    assert_eq!(record.record_length, 112);
    assert_eq!(record.major_version, 3);
    assert_eq!(record.minor_version, 0);
    assert_eq!(record.file_reference.0, 562949953456825);

    let file_ref = record.file_reference.as_mft_reference();
    assert_eq!(file_ref.entry, 35513);
    assert_eq!(file_ref.sequence, 2);

    assert_eq!(record.parent_reference.0, 562949953423304);
    let parent_ref = record.parent_reference.as_mft_reference();
    assert_eq!(parent_ref.entry, 1992);
    assert_eq!(parent_ref.sequence, 2);


    assert_eq!(record.usn, 6889306208);
    assert_eq!(format!("{}", record.timestamp), "2019-09-08 00:56:52.138160 UTC");
    assert_eq!(record.reason.bits(), 2);
    assert_eq!(record.source_info.bits(), 0);
    assert_eq!(record.security_id, 0);
    assert_eq!(record.file_attributes.bits(), 32);
    assert_eq!(record.file_name_length, 32);
    assert_eq!(record.file_name_offset, 76);
    assert_eq!(record.file_name, "CIDownloader.log");
}

#[test]
fn usn_record_v3_json_test() {
    let record_buffer: &[u8] = &[
        0x70,0x00,0x00,0x00,0x03,0x00,0x00,0x00,0xB9,0x8A,0x00,0x00,0x00,0x00,0x02,0x00,
        0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xC8,0x07,0x00,0x00,0x00,0x00,0x02,0x00,
        0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x60,0x78,0xA2,0x9A,0x01,0x00,0x00,0x00,
        0xE9,0xB6,0x4E,0x4D,0xE0,0x65,0xD5,0x01,0x02,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
        0x00,0x00,0x00,0x00,0x20,0x00,0x00,0x00,0x20,0x00,0x4C,0x00,0x43,0x00,0x49,0x00,
        0x44,0x00,0x6F,0x00,0x77,0x00,0x6E,0x00,0x6C,0x00,0x6F,0x00,0x61,0x00,0x64,0x00,
        0x65,0x00,0x72,0x00,0x2E,0x00,0x6C,0x00,0x6F,0x00,0x67,0x00,0x00,0x00,0x00,0x00
    ];

    let record = match record::UsnRecordV3::new(&mut Cursor::new(record_buffer)) {
        Ok(record) => record,
        Err(error) => panic!(error)
    };

    let json_str = serde_json::to_string(&record).unwrap();
    assert_eq!(json_str, r#"{"record_length":112,"major_version":3,"minor_version":0,"file_reference":{"u128":"562949953456825","entry":35513,"sequence":2},"parent_reference":{"u128":"562949953423304","entry":1992,"sequence":2},"usn":6889306208,"timestamp":"2019-09-08T00:56:52.138160Z","reason":"USN_REASON_DATA_EXTEND","source_info":"(empty)","security_id":0,"file_attributes":"FILE_ATTRIBUTE_ARCHIVE","file_name_length":32,"file_name_offset":76,"file_name":"CIDownloader.log"}"#);
}

#[test]
fn usn_record_v2_test() {
    let record_buffer: &[u8] = &[
        0x60,0x00,0x00,0x00,0x02,0x00,0x00,0x00,0x73,0x00,0x00,0x00,0x00,0x00,0x68,0x91,
        0x3B,0x2A,0x02,0x00,0x00,0x00,0x07,0x00,0x00,0x00,0x80,0xBC,0x04,0x00,0x00,0x00,
        0x53,0xC7,0x8B,0x18,0xC5,0xCC,0xCE,0x01,0x02,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
        0x00,0x00,0x00,0x00,0x20,0x20,0x00,0x00,0x20,0x00,0x3C,0x00,0x42,0x00,0x54,0x00,
        0x44,0x00,0x65,0x00,0x76,0x00,0x4D,0x00,0x61,0x00,0x6E,0x00,0x61,0x00,0x67,0x00,
        0x65,0x00,0x72,0x00,0x2E,0x00,0x6C,0x00,0x6F,0x00,0x67,0x00,0x00,0x00,0x00,0x00
    ];

    let record = match record::UsnRecordV2::new(&mut Cursor::new(record_buffer)) {
        Ok(record) => record,
        Err(error) => panic!(error)
    };

    assert_eq!(record.record_length, 96);
    assert_eq!(record.major_version, 2);
    assert_eq!(record.minor_version, 0);
    assert_eq!(record.file_reference.entry, 115);
    assert_eq!(record.file_reference.sequence, 37224);
    assert_eq!(record.parent_reference.entry, 141883);
    assert_eq!(record.parent_reference.sequence, 7);
    assert_eq!(record.usn, 20342374400);
    assert_eq!(format!("{}", record.timestamp), "2013-10-19 12:16:53.276040 UTC");
    assert_eq!(record.reason.bits(), 2);
    assert_eq!(record.source_info.bits(), 0);
    assert_eq!(record.security_id, 0);
    assert_eq!(record.file_attributes.bits(), 8224);
    assert_eq!(record.file_name_length, 32);
    assert_eq!(record.file_name_offset, 60);
    assert_eq!(record.file_name, "BTDevManager.log");
}

#[test]
fn usn_record_v2_json_test() {
    let record_buffer: &[u8] = &[
        0x60,0x00,0x00,0x00,0x02,0x00,0x00,0x00,0x73,0x00,0x00,0x00,0x00,0x00,0x68,0x91,
        0x3B,0x2A,0x02,0x00,0x00,0x00,0x07,0x00,0x00,0x00,0x80,0xBC,0x04,0x00,0x00,0x00,
        0x53,0xC7,0x8B,0x18,0xC5,0xCC,0xCE,0x01,0x02,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
        0x00,0x00,0x00,0x00,0x20,0x20,0x00,0x00,0x20,0x00,0x3C,0x00,0x42,0x00,0x54,0x00,
        0x44,0x00,0x65,0x00,0x76,0x00,0x4D,0x00,0x61,0x00,0x6E,0x00,0x61,0x00,0x67,0x00,
        0x65,0x00,0x72,0x00,0x2E,0x00,0x6C,0x00,0x6F,0x00,0x67,0x00,0x00,0x00,0x00,0x00
    ];

    let record = match record::UsnRecordV2::new(&mut Cursor::new(record_buffer)) {
        Ok(record) => record,
        Err(error) => panic!(error)
    };
    let json_str = serde_json::to_string(
        &record
    ).unwrap();

    assert_eq!(json_str, r#"{"record_length":96,"major_version":2,"minor_version":0,"file_reference":{"entry":115,"sequence":37224},"parent_reference":{"entry":141883,"sequence":7},"usn":20342374400,"timestamp":"2013-10-19T12:16:53.276040Z","reason":"USN_REASON_DATA_EXTEND","source_info":"(empty)","security_id":0,"file_attributes":"FILE_ATTRIBUTE_ARCHIVE | FILE_ATTRIBUTE_NOT_CONTENT_INDEXED","file_name_length":32,"file_name_offset":60,"file_name":"BTDevManager.log"}"#);
}
