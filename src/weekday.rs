use strum::Display;

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

enum Status {
    Long,
    Short,
}

// A Weekday specifies a day of the week starting from Shanbe = 0.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Display)]
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
    pub fn to_farsi_string(&self) -> String {
        self.farsi_string(Status::Long as usize)
    }
    pub fn to_farsi_short_string(&self) -> String {
        self.farsi_string(Status::Short as usize)
    }

    fn farsi_string(&self, index: usize) -> String {
        match self {
            WeekDay::Shanbeh => WEEK_DAY_FARSI[index][0],
            WeekDay::Yekshanbeh => WEEK_DAY_FARSI[index][1],
            WeekDay::Doshanbeh => WEEK_DAY_FARSI[index][2],
            WeekDay::Seshanbeh => WEEK_DAY_FARSI[index][3],
            WeekDay::Charshanbeh => WEEK_DAY_FARSI[index][4],
            WeekDay::Panjshanbeh => WEEK_DAY_FARSI[index][5],
            WeekDay::Jomeh => WEEK_DAY_FARSI[index][6],
        }
        .to_owned()
    }
}
