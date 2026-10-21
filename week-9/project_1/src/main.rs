 use std::io::Write;

fn main() {
   let mut file = std::fs::File::create("nigerian_breweries.csv").expect("create was unsuccesful");
   let header = "larger , stout , non alcoholic";
   file.write_all(header.as_bytes()).expect("could not create header");
   let row_1 = "33 export , legend , maltina\n ";
   file.write_all(row_1.as_bytes()).expect("could not create row_1");
   let row_2 = "desperados , turbo king , amstel malta\n";
   file.write_all(row_2.as_bytes()).expect("could not create row_2");
   let row_3 = "goldberg , williams , malta gold\n";
   file.write_all(row_3.as_bytes()).expect("could not create row_3");
   let row_4 = "gulder , fayrouz\n ";
   file.write_all(row_4.as_bytes()).expect("could not create row_4");
   let row_5 = "heineken,,\n";
   file.write_all(row_5.as_bytes()).expect("could not create row_5");
   let row_6 = "star,,\n";
   file.write_all(row_6.as_bytes()).expect("could not create row_6");
   println!("nigerian breweries data is saved");
} 
 


