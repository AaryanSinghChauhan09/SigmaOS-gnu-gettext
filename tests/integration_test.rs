// Integration test for SigmaOS-gnu-gettext
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use gnu_gettext as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
