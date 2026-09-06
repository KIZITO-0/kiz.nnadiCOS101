 fn main() {
	let Q1:f64 = 2.0;
    let Q2:f64 = 1.0;
    let Q3:f64 = 3.0;
    let Q4:f64 = 3.0;
    let Q5:f64 = 1.0;
    let A1:f64 = 450_000.00;
    let A2:f64 = 1_500_000.00;
    let A3:f64 = 750_000_00.00;
    let A4:f64 = 2_850_000.00;
    let A5:f64 = 250_000.00;
    //sum
    let sum:f64 = (A1*Q1)+(A2*Q2)+(A3*Q3)+(A4*Q4)+(A5*Q5);
    println!("sum is {}",sum );
    //average
    let average:f64 = sum/(Q1+Q2+Q3+Q4+Q5);
    println!("average is {}",average );
}
