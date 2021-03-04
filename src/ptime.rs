// use std::cmp::Ordering;
// use std::fmt;
// use std::ops::{Add, Sub};

// /// Represents the components of a moment in time in Persian Calendar.
// #[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
// #[cfg_attr(feature = "rustc-serialize", derive(RustcEncodable, RustcDecodable))]
// pub struct Time {
//     /// The same as `time_sec` of `time::Time`
//     pub time_sec: i32,

//     /// The same as `Time_min` of `time::Time`
//     pub time_min: i32,

//     /// The same as `time_hour` of `time::Time`
//     pub time_hour: i32,

//     /// MonthDay - [1, 31]
//     pub time_mday: i32,

//     /// Month since Farvardin - [0, 11]
//     pub time_mon: i32,

//     /// Year
//     pub time_year: i32,

//     /// Weekday since Shanbe - [0, 6]. 0 = Shanbeh, ..., 6 = Jomeh.
//     pub time_wday: i32,

//     /// YearDay since Farvardin 1 - [0, 365]
//     pub time_yday: i32,

//     /// The same as `Time_isdst` of `time::Time`
//     pub time_isdst: i32,

//     /// The same as `Time_utcoff` of `time::Time`
//     pub time_utcoff: i32,

//     /// The same as `Time_nsec` of `time::Time`
//     pub time_nsec: i32,
// }

// impl fmt::Display for Time {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}", self.to_string("yyyy-MM-ddTHH:mm:ss.ns"))
//     }
// }

// impl Add<time::Duration> for Time {
//     type Output = Time;

//     // FIXME: The timezone of `self` is different from resulting time
//     fn add(self, other: time::Duration) -> Time {
//         at_utc(self.to_timespec() + other)
//     }
// }

// impl Sub<time::Duration> for Time {
//     type Output = Time;

//     // FIXME: The timezone of `self` is different from resulting time
//     fn sub(self, other: time::Duration) -> Time {
//         at_utc(self.to_timespec() - other)
//     }
// }

// impl Sub<Time> for Time {
//     type Output = time::Duration;

//     fn sub(self, other: Time) -> time::Duration {
//         self.to_timespec() - other.to_timespec()
//     }
// }

// impl Sub<time::Time> for Time {
//     type Output = time::Duration;

//     fn sub(self, other: time::Time) -> time::Duration {
//         self.to_timespec() - other.to_timespec()
//     }
// }

// impl PartialOrd for Time {
//     fn partial_cmp(&self, other: &Time) -> Option<Ordering> {
//         self.to_timespec().partial_cmp(&other.to_timespec())
//     }
// }

// impl Ord for Time {
//     fn cmp(&self, other: &Time) -> Ordering {
//         self.to_timespec().cmp(&other.to_timespec())
//     }
// }

// impl Time {
//     /// Converts Persian calendar to Gregorian calendar
//     pub fn to_gregorian(&self) -> time::Time {
//         let year: i32;
//         let month: i32;
//         let day: i32;

//         let jdn = get_jdn(self.time_year, self.time_mon + 1, self.time_mday);

//         if jdn > 2299160 {
//             let mut l = jdn + 68569;
//             let n = 4 * l / 146097;
//             l = l - (146097 * n + 3) / 4;
//             let i = 4000 * (l + 1) / 1461001;
//             l = l - 1461 * i / 4 + 31;
//             let j = 80 * l / 2447;
//             day = l - 2447 * j / 80;
//             l = j / 11;
//             month = j + 2 - 12 * l;
//             year = 100 * (n - 49) + i + l;
//         } else {
//             let mut j = jdn + 1402;
//             let k = (j - 1) / 1461;
//             let l = j - 1461 * k;
//             let n = (l - 1) / 365 - l / 1461;
//             let mut i = l - 365 * n + 30;
//             j = 80 * i / 2447;
//             day = i - 2447 * j / 80;
//             i = j / 11;
//             month = j + 2 - 12 * i;
//             year = 4 * k + n + i - 4716;
//         }

//         time::Time {
//             time_sec: self.time_sec,
//             time_min: self.time_min,
//            time_hour: self.time_hour,
//             time_mday: day,
//             time_mon: month - 1,
//             time_year: year - 1900,
//             time_wday: get_gregorian_weekday(self.time_wday),
//             time_yday: get_gregorian_yday(year, month - 1, day),
//             time_isdst: self.time_isdst,
//             time_utcoff: self.time_utcoff,
//             time_nsec: self.time_nsec,
//         }
//     }

//     /// Returns the number of seconds since January 1, 1970 UTC
//     pub fn to_timespec(&self) -> time::Timespec {
//         self.to_gregorian().to_timespec()
//     }

//     /// Returns true if the year is a leap year
//     pub fn is_leap(&self) -> bool {
//         is_persian_leap(self.time_year)
//     }

//     /// Convert time to the local timezone
//     pub fn to_local(&self) -> Time {
//         match self.time_utcoff {
//             0 => at(self.to_timespec()),
//             _ => *self,
//         }
//     }

//     /// Convert time to the UTC
//     pub fn to_utc(&self) -> Time {
//         match self.Time_utcoff {
//             0 => *self,
//             _ => at_utc(self.to_timespec()),
//         }
//     }

//     /// Returns the formatted representation of time
//     ///     yyyy, yyy, y     year (e.g. 1394)
//     ///     yy               2-digits representation of year (e.g. 94)
//     ///     MMM              the Persian name of month (e.g. فروردین)
//     ///     MM               2-digits representation of month (e.g. 01)
//     ///     M                month (e.g. 1)
//     ///     DD               day of year (starting from 1)
//     ///     D                day of year (starting from 0)
//     ///     dd               2-digits representation of day (e.g. 01)
//     ///     d                day (e.g. 1)
//     ///     E                the Persian name of weekday (e.g. شنبه)
//     ///     e                the Persian short name of weekday (e.g. ش)
//     ///     A                the Persian name of 12-Hour marker (e.g. قبل از ظهر)
//     ///     a                the Persian short name of 12-Hour marker (e.g. ق.ظ)
//     ///     HH               2-digits representation of hour [00-23]
//     ///     H                hour [0-23]
//     ///     kk               2-digits representation of hour [01-24]
//     ///     k                hour [1-24]
//     ///     hh               2-digits representation of hour [01-12]
//     ///     h                hour [1-12]
//     ///     KK               2-digits representation of hour [00-11]
//     ///     K                hour [0-11]
//     ///     mm               2-digits representation of minute [00-59]
//     ///     m                minute [0-59]
//     ///     ss               2-digits representation of seconds [00-59]
//     ///     s                seconds [0-59]
//     ///     ns               nanoseconds
//     pub fn to_string<'a>(&'a self, format: &'a str) -> String {
//         format
//             .replace("yyyy", &self.time_year.to_string())
//             .replace("yyy", &self.time_year.to_string())
//             .replace("yy", &self.time_year.to_string()[2..])
//             .replace("y", &self.time_year.to_string())
//             .replace(
//                 "MMM",
//                 match self.time_mon {
//                     0 => "فروردین",
//                     1 => "اردیبهشت",
//                     2 => "خرداد",
//                     3 => "تیر",
//                     4 => "مرداد",
//                     5 => "شهریور",
//                     6 => "مهر",
//                     7 => "آبان",
//                     8 => "آذر",
//                     9 => "دی",
//                     10 => "بهمن",
//                     11 => "اسفند",
//                     _ => panic!("invalid month value of {}", self.time_mon),
//                 },
//             )
//             .replace("MM", &format!("{:02}", self.time_mon + 1))
//             .replace("M", &format!("{}", self.time_mon + 1))
//             .replace("DD", &format!("{}", self.time_yday + 1))
//             .replace("D", &self.time_yday.to_string())
//             .replace("dd", &format!("{:02}", self.time_mday))
//             .replace("d", &self.time_mday.to_string())
//             .replace(
//                 "E",
//                 match self.time_wday {
//                     0 => "شنبه",
//                     1 => "یک‌شنبه",
//                     2 => "دوشنبه",
//                     3 => "سه‌شنبه",
//                     4 => "چهارشنبه",
//                     5 => "پنج‌شنبه",
//                     6 => "جمعه",
//                     _ => panic!("invalid weekday value of {}", self.time_wday),
//                 },
//             )
//             .replace(
//                 "e",
//                 match self.time_wday {
//                     0 => "ش",
//                     1 => "ی",
//                     2 => "د",
//                     3 => "س",
//                     4 => "چ",
//                     5 => "پ",
//                     6 => "ج",
//                     _ => panic!("invalid weekday value of {}", self.time_wday),
//                 },
//             )
//             .replace(
//                 "A",
//                 if self.time_hour < 12 {
//                     "قبل از ظهر"
//                 } else {
//                     "بعد از ظهر"
//                 },
//             )
//             .replace("a", if self.time_hour < 12 { "ق.ظ" } else { "ب.ظ" })
//             .replace("HH", &format!("{:02}", self.time_hour))
//             .replace("H", &self.time_hour.to_string())
//             .replace("kk", &format!("{:02}", self.time_hour + 1))
//             .replace("k", &format!("{}", self.time_hour + 1))
//             .replace(
//                 "hh",
//                 &format!(
//                     "{:02}",
//                     if self.time_hour > 11 {
//                         self.time_hour - 12
//                     } else {
//                         self.time_hour
//                     } + 1
//                 ),
//             )
//             .replace(
//                 "h",
//                 &format!(
//                     "{}",
//                     if self.time_hour > 11 {
//                         self.time_hour - 12
//                     } else {
//                         self.time_hour
//                     } + 1
//                 ),
//             )
//             .replace(
//                 "KK",
//                 &format!(
//                     "{:02}",
//                     if self.time_hour > 11 {
//                         self.time_hour - 12
//                     } else {
//                         self.time_hour
//                     }
//                 ),
//             )
//             .replace(
//                 "K",
//                 &format!(
//                     "{}",
//                     if self.time_hour > 11 {
//                         self.time_hour - 12
//                     } else {
//                         self.time_hour
//                     }
//                 ),
//             )
//             .replace("mm", &format!("{:02}", self.time_min))
//             .replace("m", &self.time_min.to_string())
//             .replace("ns", &self.Time_nsec.to_string())
//             .replace("ss", &format!("{:02}", self.time_sec))
//             .replace("s", &self.time_sec.to_string())
//     }
// }

// /// Creates an empty `ptime::Time`
// pub fn empty_Time() -> Time {
//     Time {
//         time_sec: 0,
//         time_min: 0,
//         time_hour: 0,
//         time_mday: 0,
//         time_mon: 0,
//         time_year: 0,
//         time_wday: 0,
//         time_yday: 0,
//         time_isdst: 0,
//         time_utcoff: 0,
//         time_nsec: 0,
//     }
// }

// /// Converts Gregorian calendar to Persian calendar
// pub fn from_gregorian(gregorian_Time: time::Time) -> Time {
//     let mut year: i32;
//     let gy = gregorian_Time.time_year + 1900;
//     let gm = gregorian_Time.time_mon + 1;
//     let gd = gregorian_Time.time_mday;

//     let jdn: i32 = if gy > 1582 || (gy == 1582 && gm > 10) || (gy == 1582 && gm == 10 && gd > 14) {
//         ((1461 * (gy + 4800 + ((gm - 14) / 12))) / 4)
//             + ((367 * (gm - 2 - 12 * ((gm - 14) / 12))) / 12)
//             - ((3 * ((gy + 4900 + ((gm - 14) / 12)) / 100)) / 4)
//             + gd
//             - 32075
//     } else {
//         367 * gy - ((7 * (gy + 5001 + ((gm - 9) / 7))) / 4) + ((275 * gm) / 9) + gd + 1729777
//     };

//     let dep = jdn - get_jdn(475, 1, 1);
//     let cyc = dep / 1029983;
//     let rem = dep % 1029983;
//     let ycyc = if rem == 1029982 {
//         2820
//     } else {
//         let a = rem / 366;
//         (2134 * a + 2816 * (rem % 366) + 2815) / 1028522 + a + 1
//     };

//     year = ycyc + 2820 * cyc + 474;
//     if year <= 0 {
//         year -= 1;
//     }

//     let dy: f64 = (jdn - get_jdn(year, 1, 1) + 1) as f64;
//     let month: i32 = if dy <= 186f64 {
//         let mod_dy: f64 = dy / 31f64;
//         mod_dy.ceil() as i32
//     } else {
//         let mod_dy: f64 = (dy - 6f64) / 30f64;
//         mod_dy.ceil() as i32
//     } - 1;
//     let day = jdn - get_jdn(year, month + 1, 1) + 1;

//     Time {
//         time_sec: gregorian_Time.time_sec,
//         time_min: gregorian_Time.time_min,
//         time_hour: gregorian_Time.time_hour,
//         time_mday: day,
//         time_mon: month,
//         time_year: year,
//         time_wday: get_persian_weekday(gregorian_Time.time_wday),
//         time_yday: get_persian_yday(month, day),
//         time_isdst: gregorian_Time.time_isdst,
//         time_utcoff: gregorian_Time.time_utcoff,
//         time_nsec: gregorian_Time.time_nsec,
//     }
// }

// /// Creates a new instance of Persian time from Gregorian date
// pub fn from_gregorian_date(g_year: i32, g_month: i32, g_day: i32) -> Option<Time> {
//     from_gregorian_components(g_year, g_month, g_day, 0, 0, 0, 0)
// }

// /// Creates a new instance of Persian time from Persian date
// pub fn from_persian_date(p_year: i32, p_month: i32, p_day: i32) -> Option<Time> {
//     from_persian_components(p_year, p_month, p_day, 0, 0, 0, 0)
// }

// /// Creates a new instance of Persian time from Gregorian date components
// pub fn from_gregorian_components(
//     g_year: i32,
//     g_month: i32,
//     g_day: i32,
//     hour: i32,
//     minute: i32,
//     second: i32,
//     nanosecond: i32,
// ) -> Option<Time> {
//     if is_time_valid(hour, minute, second, nanosecond)
//         && is_gregorian_date_valid(g_year, g_month, g_day)
//     {
//         let Time = time::Time {
//             time_sec: second,
//            time_min: minute,
//             time_hour: hour,
//             time_mday: g_day,
//             time_mon: g_month,
//             time_year: g_year - 1900,
//             time_wday: 0,
//             time_yday: 0,
//             time_isdst: 0,
//             time_utcoff: 0,
//             time_nsec: nanosecond,
//         };
//         return Some(at_utc(Time.to_timespec()));
//     }
//     None
// }

// /// Creates a new instance of Persian time from Persian date components
// // FIXME: Calculate the weekday without converting to Gregorian calendar
// pub fn from_persian_components(
//     p_year: i32,
//     p_month: i32,
//     p_day: i32,
//     hour: i32,
//     minute: i32,
//     second: i32,
//     nanosecond: i32,
// ) -> Option<Time> {
//     if is_time_valid(hour, minute, second, nanosecond)
//         && is_persian_date_valid(p_year, p_month, p_day)
//     {
//         let mut Time = Time {
//             time_sec: second,
//             time_min: minute,
//             time_hour: hour,
//             time_mday: p_day,
//             time_mon: p_month,
//             time_year: p_year,
//             time_wday: 0,
//             time_yday: get_persian_yday(p_month, p_day),
//             time_isdst: 0,
//             time_utcoff: 0,
//             time_nsec: nanosecond,
//         };
//         Time.time_wday = get_persian_weekday(time::at_utc(Time.to_timespec()).Time_wday);
//         return Some(Time);
//     }
//     None
// }

// /// Creates a new instance of Persian time from the number of seconds since January 1, 1970 in UTC
// pub fn at_utc(clock: time::Timespec) -> Time {
//     from_gregorian(time::at_utc(clock))
// }

// /// Creates a new instance of Persian time from the number of seconds since January 1, 1970 in the local timezone
// pub fn at(clock: time::Timespec) -> Time {
//     from_gregorian(time::at(clock))
// }

// /// Creates a new instance of Persian time corresponding to the current time in UTC
// pub fn now_utc() -> Time {
//     from_gregorian(time::now_utc())
// }

// /// Creates a new instance of Persian time corresponding to the current time in the local timezone
// pub fn now() -> Time {
//     from_gregorian(time::now())
// }

// fn divider(num: i32, den: i32) -> i32 {
//     if num > 0 {
//         num % den
//     } else {
//         num - ((((num + 1) / den) - 1) * den)
//     }
// }

// fn get_jdn(year: i32, month: i32, day: i32) -> i32 {
//     let base = if year >= 0 { year - 474 } else { year - 473 };

//     let epy = 474 + (base % 2820);

//     let md = if month <= 7 {
//         (month - 1) * 31
//     } else {
//         (month - 1) * 30 + 6
//     };

//     day + md + (epy * 682 - 110) / 2816 + (epy - 1) * 365 + base / 2820 * 1029983 + 1948320
// }

// fn get_persian_weekday(wd: i32) -> i32 {
//     match wd {
//         0 => 1,
//         1 => 2,
//         2 => 3,
//         3 => 4,
//         4 => 5,
//         5 => 6,
//         6 => 0,
//         _ => panic!("invalid weekday value of {}", wd),
//     }
// }

// fn get_gregorian_weekday(wd: i32) -> i32 {
//     match wd {
//         0 => 6,
//         1 => 0,
//         2 => 1,
//         3 => 2,
//         4 => 3,
//         5 => 4,
//         6 => 5,
//         _ => panic!("invalid weekday value of {}", wd),
//     }
// }

// fn get_persian_yday(month: i32, day: i32) -> i32 {
//     [
//         0,   // Farvardin
//         31,  // Ordibehesht
//         62,  // Khordad
//         93,  // Tir
//         124, // Mordad
//         155, // Shahrivar
//         186, // Mehr
//         216, // Aban
//         246, // Azar
//         276, // Dey
//         306, // Bahman
//         336, // Esfand
//     ][month as usize]
//         + day
//         - 1
// }

// fn get_gregorian_yday(year: i32, month: i32, day: i32) -> i32 {
//     [
//         [0, 0],
//         [31, 31],
//         [59, 60],
//         [90, 91],
//         [120, 121],
//         [151, 152],
//         [181, 182],
//         [212, 213],
//         [243, 244],
//         [273, 274],
//         [304, 305],
//         [334, 335],
//     ][month as usize][is_gregorian_leap(year) as usize]
//         + day
//         - 1
// }

// fn is_persian_leap(year: i32) -> bool {
//     divider(25 * year + 11, 33) < 8
// }

// fn is_gregorian_leap(year: i32) -> bool {
//     year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
// }

// fn is_persian_date_valid(year: i32, month: i32, day: i32) -> bool {
//     if month < 0 || month > 11 {
//         return false;
//     }

//     [
//         [31, 31],
//         [31, 31],
//         [31, 31],
//         [31, 31],
//         [31, 31],
//         [31, 31],
//         [30, 30],
//         [30, 30],
//         [30, 30],
//         [30, 30],
//         [30, 30],
//         [29, 30],
//     ][month as usize][is_persian_leap(year) as usize]
//         >= day
// }

// fn is_gregorian_date_valid(year: i32, month: i32, day: i32) -> bool {
//     if month < 0 || month > 11 {
//         return false;
//     }

//     [
//         [31, 31],
//         [28, 29],
//         [31, 31],
//         [30, 30],
//         [31, 31],
//         [30, 30],
//         [31, 31],
//         [31, 31],
//         [30, 30],
//         [31, 31],
//         [30, 30],
//         [31, 31],
//     ][month as usize][is_gregorian_leap(year) as usize]
//         >= day
// }

// fn is_time_valid(hour: i32, minute: i32, second: i32, nanosecond: i32) -> bool {
//     !(hour < 0
//         || hour > 23
//         || minute < 0
//         || minute > 59
//         || second < 0
//         || second > 59
//         || nanosecond < 0
//         || nanosecond > 999999999)
// }

