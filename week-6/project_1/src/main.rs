  use std::io;
fn main() {
    let p = "Pounded Yam/Edinkaiko";
    let f = "Fried Rice & Chicken";
    let a = "Amala & Ewedu Soup";
    let e = "Eba & Egusi Soup";
    let w = "White Rice & Stew";

    let po:f32 = 3200.0;
    let fo:f32 = 3000.0;
    let ao:f32 = 2500.0;
    let eo:f32 = 2000.0;
    let wo:f32 = 2500.0;


    println!("Menu      Code              Price");
    println!("{}        P                 N3200", p);
    println!("{}         F                 N3000", f);
    println!("{}           A                 N2500", a);
    println!("{}             E                 N2000", e);
    println!("{}            W                 N2500", w);
    
    println!("Enter code: ");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Not avalid code");
    let code:String = input1.to_uppercase().trim().parse().expect("Not a valid string");

    println!("Enter quantity: ");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Not a valid number");
    let qty:f32 = input2.trim().parse().expect("Not a valid number");
    
    let mut price:f32= 0.0;

    if code == "P" && qty>0.0{
    price =  qty*po;
     println!("Total cost is N{}",price);
    }
    else if code == "F" && qty>0.00{
    price = qty*fo;
        println!("Total cost is N{}", price);
    }
    else if code == "A" && qty>0.0{
        price = qty*ao;
        println!("Total cost is N{}", price);
    }
    else if code == "E" && qty>0.0{
        price = qty*eo;
        println!("Total cost is N{}", price);
    }    
    else if code == "W" && qty>0.0{
      price = qty*wo;
        println!("Total cost is N{}", price);
    }
    else {
        println!("Invalid code or quantity");
    }
    let discount:f32 = price - (0.05*price);
    if price > 10000.00{
        println!("Discounted Total Cost is N{}", discount);
    }
}
