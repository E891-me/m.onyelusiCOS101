struct Laptop {
    brand: String,
    price: u32,
}

impl Laptop {
    fn new(brand: &str, price: u32) -> Laptop {
        Laptop {
            brand: brand.to_string(),
            price,
        }
    }

    fn calculate_cost(&self, quantity: u32) -> u32 {
        self.price * quantity
    }
}

fn main() {
    let hp = Laptop::new("HP", 650_000);
    let ibm = Laptop::new("IBM", 755_000);
    let toshiba = Laptop::new("Toshiba", 550_000);
    let dell = Laptop::new("Dell", 850_000);

    let quantity = 3;

    let total_hp = hp.calculate_cost(quantity);
    let total_ibm = ibm.calculate_cost(quantity);
    let total_toshiba = toshiba.calculate_cost(quantity);
    let total_dell = dell.calculate_cost(quantity);

    let grand_total = total_hp + total_ibm + total_toshiba + total_dell;

    // Using .brand here fixes the "never read" warning
    println!("The total cost for 3 {} laptops is: N{}", hp.brand, total_hp);
    println!("The total cost for 3 {} laptops is: N{}", ibm.brand, total_ibm);
    println!("The total cost for 3 {} laptops is: N{}", toshiba.brand, total_toshiba);
    println!("The total cost for 3 {} laptops is: N{}", dell.brand, total_dell);
   
    println!("-------------------------------------------------");
    println!("The Grand Total for the purchase is: N{}", grand_total);
}
