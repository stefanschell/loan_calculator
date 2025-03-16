pub fn calc_recurring_payment(
    num_years: i32,
    num_payments_per_year: i32,
    principal: f64,
    interest_rate: f64,
) -> f64 {
    let num_payments: i32 = num_years * num_payments_per_year;
    let interest_rate_per_payment: f64 = interest_rate / (num_payments_per_year as f64);
    principal
        * (interest_rate_per_payment * (1.0 + interest_rate_per_payment).powf(num_payments as f64))
        / ((1.0 + interest_rate_per_payment).powf(num_payments as f64) - 1.0)
}
