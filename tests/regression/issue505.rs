use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct SI32 {
    value: i32
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct SU128 {
    value: u128
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag = "type")]
enum E {
    SI32(SI32),
    SU128(SU128),
}

#[test]
#[cfg(feature = "arbitrary_precision")]
fn test_i32() {
    let s = SI32 { value: std::i32::MAX };
    let e = E::SI32(s);
    let v = dbg!(serde_json::to_string(&e).unwrap());
    let e = serde_json::from_str::<E>(&v).unwrap();

    match e {
        E::SI32(s) => assert_eq!(s, SI32 { value: std::i32::MAX }),
        _ => unreachable!(),
    }
}

#[test]
#[cfg(feature = "arbitrary_precision")]
fn test_u128() {
    let s = SU128 { value: std::u128::MAX };
    let e = E::SU128(s);
    let v = dbg!(serde_json::to_string(&e).unwrap());
    let e = serde_json::from_str::<E>(&v).unwrap();

    match e {
        E::SU128(s) => assert_eq!(s, SU128 { value: std::u128::MAX }),
        _ => unreachable!(),
    }
}
