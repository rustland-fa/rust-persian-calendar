use std::convert::TryFrom;
use strum::{Display, EnumString};

pub const WEEK_DAY_FARSI: [[&'static str; 7]; 2] = [
    [
        "شنبه",
        "یک‌شنبه",
        "دوشنبه",
        "سه‌شنبه",
        "چهارشنبه",
        "پنج‌شنبه",
        "جمعه",
    ],
    ["ش", "ی", "د", "س", "چ", "پ", "ج"],
];

enum ShowStatus {
    Long,
    Short,
}

// A Weekday specifies a day of the week starting from Shanbe = 0.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Display, EnumString)]
pub enum WeekDay {
    Shanbeh = 0,
    Yekshanbeh,
    Doshanbeh,
    Seshanbeh,
    Charshanbeh,
    Panjshanbeh,
    Jomeh,
}

impl WeekDay {
    /// Get farsi string weekday
    ///
    /// ```rust
    /// # use ptime::Weekday;
    /// assert_eq!(Weekday::Shanbeh.to_farsi_string(), "شنبه");
    /// ```
    pub fn to_farsi_string(&self) -> String {
        self.farsi_string(ShowStatus::Long)
    }
    /// Get farsi short string weekday
    ///
    /// ```rust
    /// # use ptime::Weekday;
    /// assert_eq!(Weekday::Shanbeh.to_farsi_short_string(), "ش");
    /// ```
    pub fn to_farsi_short_string(&self) -> String {
        self.farsi_string(ShowStatus::Short)
    }

    fn farsi_string(&self, status: ShowStatus) -> String {
        let week_day = WEEK_DAY_FARSI[status as usize];
        match self {
            WeekDay::Shanbeh => week_day[0],
            WeekDay::Yekshanbeh => week_day[1],
            WeekDay::Doshanbeh => week_day[2],
            WeekDay::Seshanbeh => week_day[3],
            WeekDay::Charshanbeh => week_day[4],
            WeekDay::Panjshanbeh => week_day[5],
            WeekDay::Jomeh => week_day[6],
        }
        .to_owned()
    }

    /// Get the previous weekday.
    ///
    /// ```rust
    /// # use ptime::Weekday;
    /// assert_eq!(Weekday::Shanbeh.previous(), Weekday::Jomeh);
    /// ```
    pub const fn previous(self) -> Self {
        match self {
            WeekDay::Shanbeh => WeekDay::Jomeh,
            WeekDay::Yekshanbeh => WeekDay::Shanbeh,
            WeekDay::Doshanbeh => WeekDay::Yekshanbeh,
            WeekDay::Seshanbeh => WeekDay::Doshanbeh,
            WeekDay::Charshanbeh => WeekDay::Seshanbeh,
            WeekDay::Panjshanbeh => WeekDay::Charshanbeh,
            WeekDay::Jomeh => WeekDay::Panjshanbeh,
        }
    }

    /// Get the next weekday.
    ///
    /// ```rust
    /// # use ptime::Weekday;
    /// assert_eq!(Weekday::Seshanbeh.next(), Weekday::Charshanbeh);
    /// ```
    pub const fn next(self) -> Self {
        match self {
            WeekDay::Shanbeh => WeekDay::Yekshanbeh,
            WeekDay::Yekshanbeh => WeekDay::Doshanbeh,
            WeekDay::Doshanbeh => WeekDay::Seshanbeh,
            WeekDay::Seshanbeh => WeekDay::Charshanbeh,
            WeekDay::Charshanbeh => WeekDay::Panjshanbeh,
            WeekDay::Panjshanbeh => WeekDay::Jomeh,
            WeekDay::Jomeh => WeekDay::Shanbeh,
        }
    }

    /// Get the zero-indexed number of days from Monday.
    ///
    /// ```rust
    /// # use ptime::Weekday;
    /// assert_eq!(Weekday::Shanbeh.number_days_from_shanbeh(), 0);
    /// ```
    pub const fn number_days_from_shanbeh(self) -> u8 {
        self as _
    }
}

impl TryFrom<u8> for WeekDay {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(WeekDay::Shanbeh),
            1 => Ok(WeekDay::Yekshanbeh),
            2 => Ok(WeekDay::Doshanbeh),
            3 => Ok(WeekDay::Seshanbeh),
            4 => Ok(WeekDay::Charshanbeh),
            5 => Ok(WeekDay::Charshanbeh),
            6 => Ok(WeekDay::Jomeh),
            _ => Err("invalid input data"),
        }
    }
}
