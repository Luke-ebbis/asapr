use extendr_api::prelude::*;
use asap::core::utils::distances;

/// Return string `"Hello world!"` to R.
/// @export
#[extendr]
fn hello_world() -> &'static str {
    let a = b"ATGCTCGTAGCTGCTACGTC";
    let b = b"ATCCTCGAAGCAGCTACGAC";
    let d =
        distances::percentage_difference(a.as_ref(), b.as_ref()).unwrap();

    assert!(d == 0.20);

    "Hello world!"
}

/// Return the p distance.
/// @export
#[extendr]
fn hello_distance() -> f64 {
    let a = b"ATGCTCGTAGCTGCTACGCA";
    let b = b"ATCCTCGAAGCAGCTACGAC";
    let d = distances::percentage_difference(a.as_ref(), b.as_ref()).unwrap();
    d
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod asapr;
    fn hello_world;
}
