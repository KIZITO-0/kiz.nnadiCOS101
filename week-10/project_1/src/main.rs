struct Laptop {
    brand: String,
    price: u32,
}

impl Laptop {
    fn total_cost(&self) -> u32 {
        self.price * 3

    }
}
  fn main() {
   let hp = Laptop {
    brand:String::from("HP"),
    price: 650_000,

   };

   let ibm = Laptop {
    brand:String::from("IBM"),
    price: 755_000,

   };
let toshiba = Laptop {
    brand:String::from("Toshiba"),
    price: 550_000, 
}; 

let dell = Laptop {
    brand:String::from("Dell"),
    price:850_000,
}; 
let hp_total = hp.total_cost();
let ibm_total = ibm.total_cost();
let toshiba_total = toshiba.total_cost();
let dell_total = dell.total_cost();


let grand_total = hp_total + ibm_total + toshiba_total + dell_total;


println!("{} (3 units): N{}" , hp.brand , hp_total);
println!("{} (3 units): N{}" , ibm.brand , ibm_total);
println!("{} (3 units): N{}" , toshiba.brand , toshiba_total);
println!("{} (3 units): N{}" , dell.brand , dell_total);
println!("Total cost across all brands: N{}" , grand_total);
}
