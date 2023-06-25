
#[test]
fn u8_round_trip() {
    const TEST_VALUE : u8 = 222_u8;
    let mut file = tempfile().unwrap();
    file.write_ext(&TEST_VALUE).expect("write to file");
    file.seek(std::io::SeekFrom::Start(0)).expect("seek to start of file");
    let out = file.read_ext().expect("read from file");
    assert_eq!(TEST_VALUE, out);
}

#[test]
fn u16_round_trip() {
    const TEST_VALUE : u16 = 2222_u16;
    let mut file = tempfile().unwrap();
    file.write_ext(&TEST_VALUE).expect("write to file");
    file.seek(std::io::SeekFrom::Start(0)).expect("seek to start of file");
    let out = file.read_ext().expect("read from file");
    assert_eq!(TEST_VALUE, out);
}

#[test]
fn u32_round_trip() {
    const TEST_VALUE : u32 = 222222_u32;
    let mut file = tempfile().unwrap();
    file.write_ext(&TEST_VALUE).expect("write to file");
    file.seek(std::io::SeekFrom::Start(0)).expect("seek to start of file");
    let out = file.read_ext().expect("read from file");
    assert_eq!(TEST_VALUE, out);
}

#[test]
fn u64_round_trip() {
    const TEST_VALUE : u64 = 22_222_222_222_u64;
    let mut file = tempfile().unwrap();
    file.write_ext(&TEST_VALUE).expect("write to file");
    file.seek(std::io::SeekFrom::Start(0)).expect("seek to start of file");
    let out = file.read_ext().expect("read from file");
    assert_eq!(TEST_VALUE, out);
}

#[test]
fn usize_round_trip() {
    const TEST_VALUE : usize = 2222_usize;
    let mut file = tempfile().unwrap();
    file.write_ext(&TEST_VALUE).expect("write to file");
    file.seek(std::io::SeekFrom::Start(0)).expect("seek to start of file");
    let out = file.read_ext().expect("read from file");
    assert_eq!(TEST_VALUE, out);
}

#[test]
fn string_round_trip() {
    const TEST_VALUE : &str = "TEST_VALUE";
    let mut file = tempfile().unwrap();
    file.write_ext(&String::from(TEST_VALUE)).expect("write to file");
    file.seek(std::io::SeekFrom::Start(0)).expect("seek to start of file");
    let out : String = file.read_ext().expect("read from file");
    println!("Output string: {}", out);
    assert_eq!(TEST_VALUE, &out[..]);
}

#[test]
fn vec_round_trip() {
    let test_value : Vec<&str> = vec!["TEST", "VALUE"];
    let mut file = tempfile().unwrap();
    file.write_ext(&test_value).expect("write to file");
    file.seek(std::io::SeekFrom::Start(0)).expect("seek to start of file");
    let out : Vec<String> = file.read_ext().expect("read from file");
    assert_eq!(test_value.len(), out.len());
    for (idx, original_value) in test_value.iter().enumerate() {
        assert_eq!(original_value, &out[idx]);
    }
}

#[test]
fn hash_map_round_trip() {
    let mut test_value = HashMap::new();
    test_value.insert(22_u8, "TEST");
    test_value.insert(254_u8, "VALUE");
    let mut file = tempfile().unwrap();
    file.write_ext(&test_value).expect("write to file");
    file.seek(std::io::SeekFrom::Start(0)).expect("seek to start of file");
    let out : HashMap<u8, String> = file.read_ext().expect("read from file");
    assert_eq!(test_value.len(), out.len());
    for (key, value) in test_value.into_iter() {
        assert_eq!(Some(value), out.get(&key).map(|res| &res[..]))
    }
}

use std::{io::Seek, collections::HashMap};

use tempfile::tempfile;

use crate::{WriteExt, ReadExt};