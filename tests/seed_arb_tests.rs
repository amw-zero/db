mod test_util;

// Found bug with decoding tuples with multiple values
#[test]
fn test_multi_value_tuple_insert() {
    arbtest::arbtest(test_util::property)
        .seed(0xfb7b652500000020)
        .run();
}
