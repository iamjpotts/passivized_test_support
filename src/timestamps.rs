use std::time::SystemTime;
use time::OffsetDateTime;

/// Suffix an identifier with a text representation of the current time.
pub fn named<S>(name: S) -> String where S: Into<String> {
    name.into() + "-" + &now()
}

/// A text representation of the current time, useful in identifiers such as file names.
fn now() -> String {
    let now_st = SystemTime::now();
    let now_ot: OffsetDateTime = now_st.into();

    let mut result = now_ot
        .format(&time::format_description::well_known::Rfc3339)
        .unwrap()
        .replace("-", "")
        .replace(":", "")
        .replace("T", "-");

    result.truncate(15);

    result
}

#[cfg(test)]
mod test_named {
    use std::thread::sleep;
    use std::time::Duration;

    use super::named;

    #[test]
    fn prefixed_and_timestamped() {
        const NAME: &str = "foo";

        let one = named(NAME);
        sleep(Duration::from_secs(1));
        let two = named(NAME);

        assert!(one.starts_with(NAME));
        assert!(two.starts_with(NAME));
        assert_ne!(one, two);
    }

}

#[cfg(test)]
mod test_now {
    use std::thread::sleep;
    use std::time::Duration;

    use super::now;

    #[test]
    fn varies_each_call() {
        let one = now();
        sleep(Duration::from_secs(1));
        let two = now();

        assert_ne!(one, two);
    }

}