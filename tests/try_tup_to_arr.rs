use try_tup_to_arr::*;

#[test]
fn tuple_to_arr_i8_8() {
    assert_eq!(TryTupToArr::<i8>::try_into_arr(
        (i8::MIN, i8::MAX, i8::MAX as u8, "127", i8::MAX as i16, i8::MAX as u16, i8::MIN as i32, i8::MAX as i32)),
        Ok([i8::MIN, i8::MAX, i8::MAX, i8::MAX, i8::MAX, i8::MAX, i8::MIN, i8::MAX])
    );
}

#[test]
fn tuple_to_arr_u8_8() {
    assert_eq!(TryTupToArr::<u8>::try_into_arr(
        (u8::MIN as usize, u8::MAX as isize, i8::MAX as u8, u8::MIN as i16, u8::MAX as i16, u8::MAX as u16, u8::MIN as i32, u8::MAX as i32)),
        Ok([u8::MIN, u8::MAX, i8::MAX as u8, u8::MIN, u8::MAX, u8::MAX, u8::MIN, u8::MAX])
    );
}
