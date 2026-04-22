use crate::*;

#[test]
fn test_keyed_empty() {
    let mut st = XoodyakKeyed::new(b"key", None, None, None).unwrap();
    let mut out = [0u8; 32];
    st.squeeze(&mut out);
    assert_eq!(
        out,
        [
            83, 137, 136, 225, 126, 243, 29, 108, 244, 54, 87, 107, 104, 150, 92, 180, 48, 206, 70,
            219, 244, 5, 143, 210, 43, 158, 85, 18, 198, 2, 110, 2
        ]
    );
}

#[test]
fn test_unkeyed_empty() {
    let mut st = XoodyakHash::new();
    let mut out = [0u8; 32];
    st.squeeze(&mut out);
    assert_eq!(
        out,
        [
            37, 191, 194, 243, 85, 221, 83, 252, 6, 84, 214, 202, 23, 249, 175, 158, 108, 38, 115,
            147, 79, 229, 183, 246, 82, 155, 132, 3, 114, 119, 199, 195
        ]
    );

    let mut st = XoodyakHash::new();
    let mut out = [0u8; 32];
    st.absorb(&[]);
    st.squeeze(&mut out);
    assert_eq!(
        out,
        [
            203, 49, 10, 126, 172, 235, 194, 241, 183, 89, 30, 18, 167, 86, 102, 127, 145, 32, 202,
            28, 24, 129, 133, 19, 245, 193, 104, 138, 133, 236, 122, 6
        ]
    );
}

#[test]
fn test_encrypt() {
    let mut st = XoodyakKeyed::new(b"key", None, None, None).unwrap();
    let st0 = st.clone();
    let m = b"message";
    let mut c = [0u8; 7];
    st.encrypt(&mut c, m).unwrap();

    let mut st = st0.clone();
    let mut m2 = [0u8; 7];
    st.decrypt(&mut m2, &c).unwrap();
    assert_eq!(&m[..], &m2[..]);

    let mut st = st0.clone();
    st.ratchet();
    let mut m2 = [0u8; 7];
    st.decrypt(&mut m2, &c).unwrap();
    assert_ne!(&m[..], &m2[..]);

    let c0 = c;
    let mut st = st0.clone();
    st.decrypt_in_place(&mut c);
    assert_eq!(&m[..], &c[..]);

    let mut st = st0;
    st.encrypt_in_place(&mut c);
    assert_eq!(c0, c);

    let mut tag = [0u8; 32];
    st.squeeze(&mut tag);
    assert_eq!(
        tag,
        [
            7, 189, 226, 36, 233, 112, 253, 180, 223, 91, 75, 14, 248, 93, 64, 159, 0, 4, 191, 42,
            93, 60, 40, 82, 2, 91, 244, 144, 144, 33, 104, 201
        ]
    );
}

#[test]
fn test_unkeyed_hash() {
    let mut st = XoodyakHash::new();
    let m = b"Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book. It has survived not only five centuries, but also the leap into electronic typesetting, remaining essentially unchanged. It was popularised in the 1960s with the release of Letraset sheets containing Lorem Ipsum passages, and more recently with desktop publishing software like Aldus PageMaker including versions of Lorem Ipsum.";
    st.absorb(&m[..]);
    let mut hash = [0u8; 32];
    st.squeeze(&mut hash);
    assert_eq!(
        hash,
        [
            116, 26, 155, 114, 245, 189, 177, 52, 187, 160, 108, 24, 174, 246, 47, 166, 190, 20,
            74, 202, 43, 211, 158, 253, 175, 229, 190, 18, 107, 169, 223, 34
        ]
    );
    st.absorb(&m[..]);
    let mut hash = [0u8; 32];
    st.squeeze(&mut hash);
    assert_eq!(
        hash,
        [
            253, 168, 251, 229, 80, 44, 139, 171, 181, 92, 176, 141, 24, 242, 126, 224, 17, 235,
            147, 172, 134, 178, 111, 248, 128, 135, 62, 41, 7, 39, 157, 167
        ]
    );
}

#[test]
fn test_aead() {
    let nonce = [0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    let mut st = XoodyakKeyed::new(b"key", Some(&nonce), None, None).unwrap();
    let st0 = st.clone();
    let m = b"message";
    let ad = b"ad";
    st.absorb(ad);
    let mut c = [0u8; 7 + XOODYAK_AUTH_TAG_BYTES];
    st.aead_encrypt(&mut c, Some(m)).unwrap();

    let mut st = st0.clone();
    st.absorb(ad);
    let mut m2 = [0u8; 7];
    st.aead_decrypt(&mut m2, &c).unwrap();
    assert_eq!(&m[..], &m2[..]);

    let mut st = st0;
    let mut m2 = [0u8; 7];
    let result = st.aead_decrypt(&mut m2, &m[..]);
    assert!(result.is_err());

    let mut st = XoodyakKeyed::new(b"Another key", Some(&nonce), None, None).unwrap();
    let mut m2 = [0u8; 7];
    let result = st.aead_decrypt(&mut m2, &m[..]);
    assert!(result.is_err());
}

#[test]
fn test_aead_in_place() {
    let nonce = [0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    let mut st = XoodyakKeyed::new(b"key", Some(&nonce), None, None).unwrap();
    let st0 = st.clone();

    let m = b"message";
    st.absorb(b"ad");
    let mut buf = [0u8; 7 + XOODYAK_AUTH_TAG_BYTES];
    buf[..7].copy_from_slice(m);
    st.aead_encrypt_in_place(&mut buf).unwrap();

    let mut st = st0.clone();
    let mut buf2 = buf;
    let result = st.aead_decrypt_in_place(&mut buf2);
    assert!(result.is_err());

    let mut st = st0;
    st.absorb(b"ad");
    let m2 = st.aead_decrypt_in_place(&mut buf).unwrap();
    assert_eq!(&m[..], &m2[..]);
}

#[test]
fn test_aead_detached() {
    let nonce = [0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    let mut st = XoodyakKeyed::new(b"key", Some(&nonce), None, None).unwrap();
    let st0 = st.clone();
    let m = b"message";
    st.absorb(b"ad");
    let mut c = [0u8; 7];
    let auth_tag = st.aead_encrypt_detached(&mut c, Some(m)).unwrap();

    let mut st = st0;
    let expected_tag = [
        22, 72, 159, 32, 134, 1, 184, 28, 243, 118, 113, 36, 7, 104, 72, 57,
    ];
    assert_eq!(auth_tag.as_ref(), expected_tag);
    st.absorb(b"ad");
    let mut m2 = [0u8; 7];
    st.aead_decrypt_detached(&mut m2, &expected_tag.into(), Some(&c))
        .unwrap();
    assert_eq!(&m2[..], &m[..]);
}
