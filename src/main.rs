// fn main() {
//     println!("hello ");
// }

trait Taxable {
    const TAX_RATE: f64 = 0.25;

    fn amount(&self) -> f64;

    fn tax_bill(&self) -> f64 {
        &self.amount() * Self::TAX_RATE
    }
}

#[derive(Debug)]
struct income {
    amount: f64,
}

impl Taxable for income {
    fn amount(&self) -> f64 {
        self.amount
    }
}

struct bonus {
    b_amount: f64,
}

impl Taxable for bonus {
    const TAX_RATE: f64 = 0.5;

    fn amount(&self) -> f64 {
        self.b_amount
    }
}

fn main() {
    let wer = income { amount: 10000.00 };
    println!("{:.2}", wer.tax_bill());

    let qwe = bonus { b_amount: 10000.00 };
    println!("{:.2}", qwe.tax_bill());
}
