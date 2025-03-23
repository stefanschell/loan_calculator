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
    pub schedule_end: Option<NaiveDateTime>,
    pub leftover_incoming: Option<NaiveDateTime>,
    pub leftover_amount: Option<f64>,
    pub leftover_repayment: Option<f64>,
    pub extra_win_amount: Option<f64>,
    pub extra_win_cycle: Option<Cycle>,
    pub extra_win_duration: Option<TimeDelta>,
}

pub struct ScheduleEntry {
    pub date: NaiveDateTime,
    pub loan_duration: TimeDelta,
    pub schedule_duration: TimeDelta,
    pub interest: f64,
    pub redraw: f64,
    pub repayment: f64,
    pub stashed: f64,
    pub principal: f64,
    pub stash: f64,
    pub extra_win_for_loan: f64,
    pub extra_win_for_us: f64,
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

pub fn simulate(options: &Options) -> () {
    let mut curr_date = options.schedule_start;

    let stash = 0f64;

    let extra_win_end = match options.extra_win_duration {
        Some(extra_win_duration) => Some(options.schedule_start + extra_win_duration),
        None => None,
    };

    let prev_extra_win_date = match &options.extra_win_cycle {
        Some(_) => Some(curr_date),
        None => None,
    };

    let mut schedule: Vec<ScheduleEntry> = vec![];
    let mut owing_daily_history: Vec<f64> = vec![];

    schedule.push(ScheduleEntry {
        date: curr_date,
        loan_duration: curr_date - options.loan_start,
        schedule_duration: curr_date - options.schedule_start,
        interest: 0.0,
        redraw: 0.0,
        repayment: 0.0,
        stashed: 0.0,
        principal: options.principal,
        stash: 0.0,
        extra_win_for_loan: 0.0,
        extra_win_for_us: 0.0,
    });

    let mut principal = options.principal;

    while (principal > 0f64
        || options
            .leftover_incoming
            .is_some_and(|leftover_incoming| curr_date < leftover_incoming))
        || extra_win_end.is_some_and(|extra_win_end| curr_date < extra_win_end)
    {
        curr_date = curr_date + Duration::days(1);

        let maturity_is_today = match options.schedule_end {
            Some(schedule_end) => curr_date >= schedule_end,
            None => false,
        };

        let mut curr_interest: Option<f64> = None;
        let mut curr_redraw: Option<f64> = None;
        let mut curr_repayment: Option<f64> = None;
        let mut curr_stashed: Option<f64> = None;
        let mut curr_extra_win_for_loan: Option<f64> = None;
        let mut curr_extra_win_for_us: Option<f64> = None;

        // interest
        // note: we keep track of the amount owning on a daily basis,
        //       back to the previous interest calculation

        owing_daily_history.push((principal - options.offset).max(0f64));
    }
}
