pub mod mortgage {

    extern crate chrono;
    use chrono::format::ParseError;
    use chrono::{Months, NaiveDate};
    use std::collections::BTreeMap;
    use serde::{Serialize, Deserialize};

    //https://mortgage-calculator.ru/формула-расчета-ипотеки/
    type Payments = BTreeMap<String, f32>;
    #[derive(Serialize)]
    pub struct PaymentShedule {
        payment_details: BTreeMap<String, Payments>
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Mortgage {
        pub amount: u64,
        mortgage_rate: f32,
        period: u16,
        taking_date: String,
        payment_type: String,
        initial_payment: u64,
    }

    impl Mortgage {
        #[allow(dead_code)]
        pub fn new(
            amount: u64,
            mortgage_rate: f32,
            period: u16,
            taking_date: String,
            payment_type: String,
            initial_payment: u64,
        ) -> Self {
            Self {
                amount: amount,
                mortgage_rate: mortgage_rate,
                period: period,
                taking_date: taking_date,
                payment_type: payment_type,
                initial_payment: initial_payment,
            }
        }

        fn calculate_annuity_monthly_payment(&self, amount: Option<f32>) -> f32 {
            //Расчет ежемесечного ануитентного платежа
            let monthly_payment: f32;
            let months = 12 as f32;
            let period = (self.period as f32) * months;
            let mut amount = match amount {
                Some(i) => i,
                None => self.amount as f32,
            };
            let mounth_rate = self.mortgage_rate / months / 100.0;
            let full_period_rate = (1.0 + mounth_rate).powf(period);
            if self.amount as f32 == amount {
                amount = (self.amount - self.initial_payment) as f32;
            }
            monthly_payment = amount * mounth_rate * full_period_rate / (full_period_rate - 1.0);
            monthly_payment
        }

        fn calculate_differentiated_monthly_payment(&self, amount: Option<f32>) -> f32 {
            //Расчет ежемесечного диффиринциального платежа
            let monthly_payment =
                self.calculate_monthly_repayment_debt() + self.calculate_percent_part(amount);
            monthly_payment
        }

        //Тут ошибка?
        fn calculate_percent_part(&self, amount: Option<f32>) -> f32 {
            //Расчет суммы процентов которые начисляются за месяц
            //Одинаково для аннуитентных и диффиринциальных платежей
            let months = 12 as f32;
            let mounth_rate = self.mortgage_rate / months / 100.0;
            let amount = match amount {
                Some(i) => i,
                None => self.amount as f32,
            };
            let percent_part;
            if amount == self.amount as f32 {
                percent_part = (amount - self.initial_payment as f32) * mounth_rate;
            } else {
                percent_part = amount * mounth_rate;
            }
            percent_part
        }

        fn calculate_annuity_body_part(&self, amount: Option<f32>) -> f32 {
            //Расчет основной части кредита в ежемесечном платеже
            let mounthly_payment = self.calculate_annuity_monthly_payment(Some(self.amount as f32));
            let percent_part = self.calculate_percent_part(amount);
            let body_part = mounthly_payment - percent_part;
            body_part
        }

        fn calculate_overpayment(&self) -> f32 {
            //Расчет переплаты
            let months = 12 as f32;
            let period = (self.period as f32) * months;
            let mounthly_payment: f32;
            if self.payment_type == "annuity".to_string() {
                mounthly_payment = self.calculate_annuity_monthly_payment(None);
            } else if self.payment_type == "differentiated".to_string() {
                mounthly_payment = self.calculate_differentiated_monthly_payment(None);
            } else {
                println!("Указан некорректный тип платежа!");
                mounthly_payment = 0.0;
            }
            let overpayment = mounthly_payment * period - self.amount as f32;
            overpayment
        }

        fn calculate_monthly_repayment_debt(&self) -> f32 {
            //Только для диффиринциальных платежей ?
            let months = 12 as f32;
            let period = (self.period as f32) * months;
            let monthly_repayment_debt = self.amount as f32 / period;
            monthly_repayment_debt
        }

        fn next_payment_date(&self, month: u16) -> Option<NaiveDate> {
            //получение даты платежа
            let taking_date = NaiveDate::parse_from_str(&self.taking_date, "%d.%m.%Y")
                .expect("Введен неверный формат даты!");
            let next_payment_date = taking_date.checked_add_months(Months::new(month as u32));
            next_payment_date
        }

        pub fn show_payment_schedule(&self) -> PaymentShedule {
            //Распечатывает график платежей
            let mut p_d: BTreeMap<String, Payments> = BTreeMap::new();
            let mut payments:Payments = BTreeMap::new();
            let mut date: String;
            let mut mounthly_payment: f32;
            let mut percent_part: f32;
            let mut body_part: f32;
            let mut debt_on_date: f32;
            if self.payment_type == "annuity".to_string() {
                for m in 1..(self.period * 12 + 1) {
                    date = self.next_payment_date(m).unwrap().to_string();
                    debt_on_date = self.debt_on_date(m);
                    body_part = self.calculate_annuity_body_part(Some(debt_on_date));
                    percent_part = self.calculate_percent_part(Some(debt_on_date));
                    mounthly_payment = self.calculate_annuity_monthly_payment(None);
                    payments.insert("mounthly_payment".to_string(), mounthly_payment);
                    payments.insert("percent_part".to_string(), percent_part);
                    payments.insert("body_part".to_string(), body_part);
                    payments.insert("remaining_debt".to_string(), debt_on_date);
                    p_d.insert(date, payments.clone());
                }
            }
            PaymentShedule {
                payment_details: p_d,
            }
        }

        fn debt_on_date(&self, month: u16) -> f32 {
            //Расчет оставшейся суммы задолженности на конкретный (порядковый) месяц
            let months = 12 as f32;
            let amount = self.amount as f32 - self.initial_payment as f32;
            let mut debt: f32 = amount + amount * self.mortgage_rate / months / 100.0;
            let mounthly_payment = self.calculate_annuity_monthly_payment(None);
            for _ in 0..month {
                if debt >= mounthly_payment {
                    debt = debt - mounthly_payment;
                    debt = debt + debt * self.mortgage_rate / months / 100.0;
                } else if debt < mounthly_payment {
                    debt = debt + debt * self.mortgage_rate / months / 100.0;
                }
            }
            debt
        }

        pub fn calculate_total_amount(&self) -> f32 {
            self.amount as f32 + self.calculate_overpayment()
        }
    }
}
