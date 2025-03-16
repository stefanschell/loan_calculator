use chrono::{Datelike, Duration, NaiveDateTime, TimeDelta};

pub enum Cycle {
    Fortnightly,
    MonthlyAverage,
    MonthlyFirstOfMonth,
    MonthlyEndOfMonth,
    YEARLY,
}

pub struct Options {
    pub loan_start: NaiveDateTime,
    pub principal: f64,
    pub offset: f64,
    pub schedule_start: NaiveDateTime,
    pub interest_rate: f64,
    pub prev_interest_date: NaiveDateTime,
    pub interest_cycle: Cycle,
    pub repayment: f64,
    pub prev_repayment_date: NaiveDateTime,
    pub repayment_cycle: Cycle,
    pub repayment_use_stash: bool,
    pub schedule_end: NaiveDateTime,
    pub leftover_incoming: Option<NaiveDateTime>,
    pub leftover_amount: Option<f64>,
    pub leftover_repayment: Option<f64>,
    pub extra_win_amount: Option<f64>,
    pub extra_win_cycle: Option<Cycle>,
    pub extra_win_duration: Option<TimeDelta>,
}

fn increment_date(date: &NaiveDateTime, cycle: &Cycle) -> NaiveDateTime {
    match cycle {
        Cycle::Fortnightly => *date + Duration::weeks(2),

        Cycle::MonthlyAverage => *date + Duration::seconds(((365. / 12.) * 24. * 60. * 60.) as i64),

        Cycle::MonthlyFirstOfMonth => {
            let mut new_date: NaiveDateTime = (*date).with_day(1).unwrap(); // first of current month
            new_date = (new_date + Duration::days(31)).with_day(1).unwrap(); // first of next month
            new_date
        }

        Cycle::MonthlyEndOfMonth => {
            let duration_max_month: TimeDelta = Duration::days(31);
            let duration_day: TimeDelta = Duration::days(1);

            let date_is_last_day_of_month: bool = (*date + duration_day).month() != date.month();

            if date_is_last_day_of_month {
                // normal behavior: we are calling this method with date being at the end of the month,
                //                  thus, we want the end of next month
                let mut new_date: NaiveDateTime = (*date).with_day(1).unwrap(); // first of current month
                new_date = (new_date + duration_max_month).with_day(1).unwrap(); // first of next month
                new_date = (new_date + duration_max_month).with_day(1).unwrap(); // first of month after next
                new_date = new_date - duration_day; // end of next month
                new_date
            } else {
                // special behavior: we are calling this method with date not being at the end of the month,
                //                   thus, we want the end of this month
                let mut new_date: NaiveDateTime = (*date).with_day(1).unwrap(); // first of current month
                new_date = (new_date + duration_max_month).with_day(1).unwrap(); // first of next month
                new_date = new_date - duration_day; // end of this month
                new_date
            }
        }

        Cycle::YEARLY => *date + Duration::days(365),
    }
}

pub fn simulate(options: &Options) -> () {}
