use generics::test_workdir::remove_test_work_dir_path;

use crate::common::setup::get_core_test;

mod common;

#[test]
fn test_core_wrapper() {
    let dir = "retro_core..test_core";

    let core = get_core_test(dir).unwrap();

    remove_test_work_dir_path(dir);
}
