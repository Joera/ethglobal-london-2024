#![allow(non_snake_case)]
use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;
use serde_json;
use serde_json::{Value, json};
use mem_types::{MemSubscription, MemContentObject};
use std::fs;


module_manifest!();

mod types;
mod dune;
mod vault;

pub fn main() {}

#[marine]
pub fn add(subscription: MemSubscription) ->  Vec<MemContentObject> {

    let mut objects : Vec<MemContentObject> = vec!();
    let mut errors : Vec<String> = vec!();
    let target_path = vault::vault_path("mem_response");

    let r = dune::get(&subscription.campaign.dune_query_id, &target_path);

 //   let json_string = serde_json::to_string(&r).unwrap();

    if r.success {

        let r_result = fs::read_to_string(target_path.clone()).unwrap();
        let r_json : Value = serde_json::from_str(&r_result).unwrap();

        let rows = &r_json["result"]["rows"];

        // let mut total_safes = 0;

        // let mut weeks: Vec<types::Week> = vec!();

        // for row in rows.as_array().iter() {

        //     for r in row.iter() {
  
        //        // total_safes += r["num_safes"].as_u64().unwrap();

        //         let w = types::Week {
        //             "date": r["week"].to_string(),
        //             "value": "500".to_string()
        //         };

        //         weeks.push(w);

        //     }
        
        // }

        let mockedData = json!([
            { "date": "2024-03-11 00:00:00.000 UTC", "value": 7380335 },
            { "date": "2024-03-04 00:00:00.000 UTC", "value": 7202740 },
            { "date": "2024-02-26 00:00:00.000 UTC", "value": 6923208 },
            { "date": "2024-02-19 00:00:00.000 UTC", "value": 6662470 },
            { "date": "2024-02-12 00:00:00.000 UTC", "value": 6349773 },
            { "date": "2024-02-05 00:00:00.000 UTC", "value": 6207893 },
            { "date": "2024-01-29 00:00:00.000 UTC", "value": 6105595 },
            { "date": "2024-01-22 00:00:00.000 UTC", "value": 5990306 }
        ]);
          

        
        
        let o = MemContentObject {
            subscription,
            data_object: serde_json::to_string(&rows).unwrap()
        };

        objects.push(o);
    
    } else {

        errors.push(r.error)

    }

    // let r1 = dune::post(&subscription.campaign.dune_query_id, &target_path);

    // if r1.success {

    //     let r1_result = fs::read_to_string(target_path.clone()).unwrap();
    //     let r1_json : Value = serde_json::from_str(&r1_result).unwrap();

    //     let r2 = dune::get(r1_json["execution_id"].to_string(), &target_path);
    
    //     if r2.success {

    //         let r2_result = fs::read_to_string(target_path.clone()).unwrap();
    //         let r2_json : Value = serde_json::from_str(&r2_result).unwrap();
            
    //         let o = MemContentObject {
    //             subscription,
    //             data_object: serde_json::to_string(&r2_json["data"]).unwrap()
    //         };

    //         objects.push(o);
    //     } 

    // }

    objects

}
