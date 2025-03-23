mod planner;
mod simulator;

use chrono::{Duration, Local, NaiveDateTime};
use simulator::Options;

fn main() {
    let num_years: i32 = 25;
    let num_payments_per_year: i32 = 12;
    let principal: f64 = 1e6;
    let interest_rate: f64 = 0.065;

    let repayment: f64 =
        planner::calc_recurring_payment(num_years, num_payments_per_year, principal, interest_rate);

    println!("Recurring payment is: {}", repayment);

    let loan_start: NaiveDateTime = Local::now().naive_local();
    let loan_end: NaiveDateTime = loan_start + Duration::days((365 * num_years) as i64);

    let options: Options = simulator::Options {
        loan_start,
        principal: 1e6,
        offset: 0.0,
        schedule_start: loan_start,
        interest_rate: 0.065,
        prev_interest_date: loan_start,
        interest_cycle: simulator::Cycle::MonthlyEndOfMonth,
        repayment,
        prev_repayment_date: loan_start,
        repayment_cycle: simulator::Cycle::Fortnightly,
        repayment_use_stash: false,
        schedule_end: Some(loan_end),
        leftover_incoming: None,
        leftover_amount: None,
        leftover_repayment: None,
        extra_win_amount: None,
        extra_win_cycle: None,
        extra_win_duration: None,
    };

    let df = simulator::simulate(&options);
}
