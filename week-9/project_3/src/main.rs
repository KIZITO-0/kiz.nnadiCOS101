use std::io::Write;
use std::fs;
fn  main() {
   let mut file = fs::File::create("convicted-ministers.csv").expect("create was unsuccessful");
   file.write_all("EFCC convicted ministers".as_bytes()).expect("could not write");
   let header = "comissioner , mimistry , zone";
   file.write_all(header.as_bytes()).expect("could not write");
   let names = vec!["aigbogun alamba daudu" , "murtala afeez bendu" , "okorocha calistus ogbona" , "adewale jimoh akanbi" , "osazuwa faith etieye"];
   let ministry = vec!["internal affairs" , "justice" , "defence" , "power & steel" , "petroleum"];
   let zone = vec!["south west" , "north east" , "south south" , "south west" , "south east"];
   for k in 0..names.len() {
       let row = format!("{} , {} , {}", names[k] , ministry[k] , zone[k]);
       file.write_all(row.as_bytes()).expect("could not write");

   }
    println!("data written to file");
}  

