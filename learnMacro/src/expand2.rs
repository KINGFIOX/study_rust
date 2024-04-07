macro_rules! capture_then_what_is {
    (#[$m:meta]) => {what_is!(#[$m])};
}

macro_rules! what_is {
    (#[no_mangle]) => {"no_mangle attribute"};
    (#[inline]) => {"inline attribute"};
    ($($tts:tt)*) => {concat!("something else (", stringify!($($tts)*), ")")};
}

fn main() {
    println!(
        "{}\n{}\n{}\n{}",
        what_is!(#[no_mangle]),
        what_is!(#[inline]),
        // no_mangle attribute
        // inline attribute
        capture_then_what_is!(#[no_mangle]),
        capture_then_what_is!(#[inline]),
        // something else (#[no_mangle])
        // something else (#[inline])
    );
}
