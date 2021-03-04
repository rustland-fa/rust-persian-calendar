use strum::Display;

pub const MONTH_FARSI: [&'static str; 12] = [
    "فروردین",
    "اردیبهشت",
    "خرداد",
    "تیر",
    "مرداد",
    "شهریور",
    "مهر",
    "آبان",
    "آذر",
    "دی",
    "بهمن",
    "اسفند",
];

pub const MONTH_DARI: [&'static str; 12] = [
    "حمل",
    "ثور",
    "جوزا",
    "سرطان",
    "اسد",
    "سنبله",
    "میزان",
    "عقرب",
    "قوس",
    "جدی",
    "دلو",
    "حوت",
];

// A Month specifies a month of the year starting from Farvardin = 1.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Display)]
pub enum Month {
    Farvardin = 1,
    Ordibehesht,
    Khordad,
    Tir,
    Mordad,
    Shahrivar,
    Mehr,
    Aban,
    Azar,
    Dey,
    Bahman,
    Esfand,
}
impl Month {
    pub fn farsi_string(&self) -> String {
        let r = match self {
            Month::Farvardin => MONTH_FARSI[0],
            Month::Ordibehesht => MONTH_FARSI[1],
            Month::Khordad => MONTH_FARSI[2],
            Month::Tir => MONTH_FARSI[3],
            Month::Mordad => MONTH_FARSI[4],
            Month::Shahrivar => MONTH_FARSI[5],
            Month::Mehr => MONTH_FARSI[6],
            Month::Aban => MONTH_FARSI[7],
            Month::Azar => MONTH_FARSI[8],
            Month::Dey => MONTH_FARSI[9],
            Month::Bahman => MONTH_FARSI[10],
            Month::Esfand => MONTH_FARSI[11],
        };
        return r.to_owned();
    }
    fn month_number(i: i32) -> Month {
        if i == 1 {
            return Month::Farvardin;
        } else if i == 2 {
            return Month::Ordibehesht;
        } else if i == 3 {
            return Month::Khordad;
        } else if i == 4 {
            return Month::Tir;
        } else if i == 5 {
            return Month::Mordad;
        } else if i == 6 {
            return Month::Shahrivar;
        } else if i == 7 {
            return Month::Mehr;
        } else if i == 8 {
            return Month::Aban;
        } else if i == 9 {
            return Month::Azar;
        } else if i == 10 {
            return Month::Dey;
        } else if i == 11 {
            return Month::Bahman;
        } else if i == 12 {
            return Month::Esfand;
        } else {
            panic!("invalid input");
        }
    }
}
// List of Dari months in Persian calendar.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Display)]
pub enum MonthDari {
    Hamal = 1,
    Sur,
    Jauza,
    Saratan,
    Asad,
    Sonboleh,
    Mizan,
    Aqrab,
    Qos,
    Jady,
    Dolv,
    Hut,
}
impl MonthDari {
    pub fn farsi_string(&self) -> String {
        let r = match self {
            MonthDari::Hamal => MONTH_DARI[0],
            MonthDari::Sur => MONTH_DARI[1],
            MonthDari::Jauza => MONTH_DARI[2],
            MonthDari::Saratan => MONTH_DARI[3],
            MonthDari::Asad => MONTH_DARI[4],
            MonthDari::Sonboleh => MONTH_DARI[5],
            MonthDari::Mizan => MONTH_DARI[6],
            MonthDari::Aqrab => MONTH_DARI[7],
            MonthDari::Qos => MONTH_DARI[8],
            MonthDari::Jady => MONTH_DARI[9],
            MonthDari::Dolv => MONTH_DARI[10],
            MonthDari::Hut => MONTH_DARI[11],
        };
        return r.to_owned();
    }

    fn month_number(i: i32) -> MonthDari {
        if i == 1 {
            return MonthDari::Hamal;
        } else if i == 2 {
            return MonthDari::Sur;
        } else if i == 3 {
            return MonthDari::Jauza;
        } else if i == 4 {
            return MonthDari::Saratan;
        } else if i == 5 {
            return MonthDari::Asad;
        } else if i == 6 {
            return MonthDari::Sonboleh;
        } else if i == 7 {
            return MonthDari::Mizan;
        } else if i == 8 {
            return MonthDari::Aqrab;
        } else if i == 9 {
            return MonthDari::Qos;
        } else if i == 10 {
            return MonthDari::Jady;
        } else if i == 11 {
            return MonthDari::Dolv;
        } else if i == 12 {
            return MonthDari::Hut;
        } else {
            panic!("invalid input");
        }
    }
}
