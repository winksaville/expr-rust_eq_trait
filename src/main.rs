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

// Don't use all of the fields
impl PartialEq for ExprEqu {
    fn eq(&self, other: &Self) -> bool {
        self.s == other.s
    }
}

fn main() {
    let mut eet1 = ExprEqu::new();
    let mut eet2 = ExprEqu::new();

    eet1.n = 1;
    eet1.s = "1".to_owned();

    eet2.n = 2;
    eet2.s = "2".to_owned();

    println!("eet1: {eet1:?}!");
    println!("eet2: {eet2:?}!");
    assert_eq!(eet1, eet1);
    assert_ne!(eet1, eet2);
    assert_ne!(eet2, eet1);
}
