fn main () {
	// define the vales for quantity
	let tf:f64 = 2.0;
	let mf:f64 = 1.0;
	let hf:f64 = 3.0;
	let df:f64 = 3.0;
	let af:f64 = 1.0;

    // define the values for amount
    let tx:f64 = 450000.0;
    let mx:f64 = 1500000.0;
    let hx:f64 = 750000.0;
    let dx:f64 = 2850000.0;
    let ax:f64 = 250000.00;

    // sum
    let s = (tf * tx) + (mf * mx) + (hf * hx) + (df * dx) + (af * ax);
    println!("Sum is {}", s);
    // average
    let a = s/(tf + mf + hf + df + af);
    println!("Average is {}", a); 
}