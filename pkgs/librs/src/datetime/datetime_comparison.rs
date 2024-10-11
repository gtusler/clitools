use chrono::{DateTime, Datelike, Utc};

/// Are the two dates in the same year?
pub fn is_same_year(orig: DateTime<Utc>, comp: DateTime<Utc>) -> bool {
    let orig_date = orig.naive_utc();
    let comp_date = comp.naive_utc();

    orig_date.year() == comp_date.year()
}

/// Are the two dates the same year and month?
pub fn is_same_ym(orig: DateTime<Utc>, comp: DateTime<Utc>) -> bool {
    let orig_date = orig.naive_utc();
    let comp_date = comp.naive_utc();

    if orig_date.year() == comp_date.year() {
        orig_date.month() == comp_date.month()
    } else {
        false
    }
}

/// Are the two dates the same year, month and day?
pub fn is_same_ymd(orig: DateTime<Utc>, comp: DateTime<Utc>) -> bool {
    let orig_date = orig.naive_utc();
    let comp_date = comp.naive_utc();

    if orig_date.year() == comp_date.year() {
        if orig_date.month() == comp_date.month() {
            orig_date.day() == comp_date.day()
        } else {
            false
        }
    } else {
        false
    }
}
