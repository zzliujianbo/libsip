use libsip::{headers::parse::parse_subject_header, Header};

#[test]
fn write() {
    let header = Header::Subject("Softphone 1.0".into());
    assert_eq!("Subject: Softphone 1.0".to_string(), format!("{}", header));
}

#[test]
fn read() {
    let remains = vec![];
    let header = Header::Subject("Softphone 1.0".into());
    assert_eq!(
        Ok((remains.as_ref(), header)),
        parse_subject_header(b"Subject: Softphone 1.0\r\n")
    );
}
