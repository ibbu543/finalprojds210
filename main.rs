extern crate csv;

use std::error::Error;
use csv::StringRecord;
use kmeans::*;

#[derive(Debug)]
struct DataFrame {
   header: csv::StringRecord,
   temp: Vec<f64>,
   hum: Vec<f64>,
   windspeed: Vec<f64>,
   cnt: Vec<f64>
 }


impl DataFrame {

    fn new() -> DataFrame {
        DataFrame {
            header: csv::StringRecord::new(),
            temp: Vec::new(),
            hum: Vec::new(),
            windspeed: Vec::new(),
            cnt: Vec::new(),
        }
     }

     fn read_csv(filepath: &str, has_headers: bool) -> DataFrame {
        
         let file = std::fs::File::open("./day.csv").unwrap();
         let mut rdr = csv::ReaderBuilder::new()
            .has_headers(has_headers)
            .from_reader(file);

         let mut data_frame = DataFrame::new();

        
         for result in rdr.records().into_iter() {
            let record = result.unwrap();
            data_frame.push(&record);
         }
         return data_frame;
      }

      fn push(&mut self, row: &csv::StringRecord) {
       
          self.temp.push(row[0].parse().unwrap());
       
          self.hum.push(row[1].parse().unwrap());
         
          self.windspeed.push(row[2].parse().unwrap());
          
          self.cnt.push(row[3].parse().unwrap())
      }
}


fn main() {
   let mut data = DataFrame::read_csv("path to file", true);
   let samples = &mut data.temp;
   samples.append(&mut data.hum);
   samples.append(&mut data.windspeed);
   samples.append(&mut data.cnt);

   let sample_count = 731;
   let dimensions = 4;
   let k = 5;
   let max_iter = 500;

   let kmean = KMeans::new(samples.to_vec(), sample_count, dimensions);
   let result = kmean.kmeans_lloyd(k, max_iter, KMeans::init_kmeanplusplus, &KMeansConfig::default());

   println!("Centroids: {:?}", result.centroids);
   println!("Cluster-Assignments: {:?}", result.assignments);
   println!("Error: {}", result.distsum);

}