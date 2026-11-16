//! Timestamps: whole seconds since the Unix epoch, written as ISO-8601 UTC.
//!
//! No external crates: the civil-date conversion follows Howard Hinnant's
//! `days_from_civil` / `civil_from_days` algorithms.

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Timestamp(pub i64);

impl Timestamp {
    pub fn now() -> Self {
        let secs = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);
        Timestamp(secs)
    }

    pub fn from_ymd_hms(y: i64, m: u32, d: u32, hh: u32, mm: u32, ss: u32) -> Self {
        let days = days_from_civil(y, m, d);
        Timestamp(days * 86_400 + hh as i64 * 3600 + mm as i64 * 60 + ss as i64)
    }

    pub fn from_ymd(y: i64, m: u32, d: u32) -> Self {
        Self::from_ymd_hms(y, m, d, 0, 0, 0)
    }

    pub fn plus_secs(self, s: i64) -> Self {
        Timestamp(self.0 + s)
    }

    pub fn plus_days(self, d: i64) -> Self {
        Timestamp(self.0 + d * 86_400)
    }

    /// Accepts `YYYY-MM-DD`, `YYYY-MM-DDTHH:MM`, `YYYY-MM-DDTHH:MM:SS`,
    /// each with an optional trailing `Z`.
    pub fn parse(s: &str) -> Option<Self> {
        let s = s.trim();
        let s = s.strip_suffix('Z').unwrap_or(s);
        let (date, time) = match s.split_once('T') {
            Some((d, t)) => (d, Some(t)),
            None => (s, None),
        };
        let mut dp = date.split('-');
        let y: i64 = dp.next()?.parse().ok()?;
        let m: u32 = dp.next()?.parse().ok()?;
        let d: u32 = dp.next()?.parse().ok()?;
        if dp.next().is_some() || !(1..=12).contains(&m) || !(1..=31).contains(&d) {
            return None;
        }
        let (hh, mm, ss) = match time {
            None => (0, 0, 0),
            Some(t) => {
                let mut tp = t.split(':');
                let hh: u32 = tp.next()?.parse().ok()?;
                let mm: u32 = tp.next()?.parse().ok()?;
                let ss: u32 = match tp.next() {
                    Some(x) => x.parse().ok()?,
                    None => 0,
                };
                if tp.next().is_some() || hh > 23 || mm > 59 || ss > 60 {
                    return None;
                }
                (hh, mm, ss)
            }
        };
        Some(Self::from_ymd_hms(y, m, d, hh, mm, ss))
    }

    pub fn ymd(self) -> (i64, u32, u32) {
        civil_from_days(self.0.div_euclid(86_400))
    }

    pub fn hms(self) -> (u32, u32, u32) {
        let r = self.0.rem_euclid(86_400);
        ((r / 3600) as u32, ((r % 3600) / 60) as u32, (r % 60) as u32)
    }

    /// Hour of day, 0..=23.
    pub fn hour(self) -> u32 {
        self.hms().0
    }
}

/// Shortest exact form: a date if the time is midnight, minutes if seconds are zero.
impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (y, m, d) = self.ymd();
        let (hh, mm, ss) = self.hms();
        if hh == 0 && mm == 0 && ss == 0 {
            write!(f, "{y:04}-{m:02}-{d:02}")
        } else if ss == 0 {
            write!(f, "{y:04}-{m:02}-{d:02}T{hh:02}:{mm:02}")
        } else {
            write!(f, "{y:04}-{m:02}-{d:02}T{hh:02}:{mm:02}:{ss:02}")
        }
    }
}

fn days_from_civil(y: i64, m: u32, d: u32) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = (if y >= 0 { y } else { y - 399 }) / 400;
    let yoe = y - era * 400;
    let m = m as i64;
    let d = d as i64;
    let mp = if m > 2 { m - 3 } else { m + 9 };
    let doy = (153 * mp + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}

fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719_468;
    let era = (if z >= 0 { z } else { z - 146_096 }) / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = (if mp < 10 { mp + 3 } else { mp - 9 }) as u32;
    (if m <= 2 { y + 1 } else { y }, m, d)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn epoch() {
        assert_eq!(Timestamp::from_ymd(1970, 1, 1).0, 0);
        assert_eq!(Timestamp(0).to_string(), "1970-01-01");
    }

    #[test]
    fn roundtrip_forms() {
        for s in ["2026-11-16", "2026-11-16T08:30", "2026-02-28T23:59:07", "2000-02-29"] {
            let t = Timestamp::parse(s).unwrap();
            assert_eq!(t.to_string(), s);
        }
        assert_eq!(
            Timestamp::parse("2026-11-16T00:00:00Z").unwrap().to_string(),
            "2026-11-16"
        );
    }

    #[test]
    fn rejects_garbage() {
        assert!(Timestamp::parse("yesterday").is_none());
        assert!(Timestamp::parse("2026-13-01").is_none());
        assert!(Timestamp::parse("2026-01-01T25:00").is_none());
    }

    #[test]
    fn many_days_roundtrip() {
        let start = Timestamp::from_ymd(1990, 1, 1);
        for i in 0..20_000 {
            let t = start.plus_days(i).plus_secs(i * 37 % 86_400);
            assert_eq!(Timestamp::parse(&t.to_string()), Some(t));
        }
    }
}
