use crate::common::util::*;

#[test]
fn test_default_mode() {
    // test the default mode

    // accept some reasonable default
    new_ucmd!().args(&["abc/def"]).succeeds().no_stdout();

    // fail on long inputs
    new_ucmd!()
        .args(&["test".repeat(20000)])
        .fails()
        .no_stdout();
}
