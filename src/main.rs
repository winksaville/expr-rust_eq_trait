#[derive(Debug)]
pub struct ExprEqu {
    pub n: i64,
    pub s: String,
}

impl ExprEqu {
    fn new() -> ExprEqu {
        ExprEqu {
            n: 0,
            s: "".to_owned(),
        }
    }
}
fn main() {
    let eet = ExprEqu::new();

    println!("eet: {eet:?}!");
}
