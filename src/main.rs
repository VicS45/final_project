// RUST FINAL PROJECT:
// A program I wrote that makes a GET request to an API,
// parses the json data, and displays the response in a user readable format

use  serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize,Debug)]

struct CoffeeStruct
{
    file: String

}

/// (1) Hook up tokio for async runtime capabilities                    (1)

/// (4) call functions in main                                          (4) 

#[tokio::main]
async fn main() {

    println!("");
    println!("**********************");
    println!("The raw data:");
    println!("**********************");
    println!("");

    let _rawdata = raw().await;

    println!("");
    println!("**********************");
    println!("The formatted data:");
    println!("**********************");
    println!("");

    let formatted_data = formatted().await;
    println!("{:#?}", formatted_data);

    println!("");
    
    }

/// (2) "raw" function -                                                                      (2)
/// create new request client, issue GET request to api, 
/// send the request, await response, turn response body into
/// string, await that operation, save string in "coffee" variable 
//  and then print json string containing array 
/// *** await is used after async methods to wait for responses until 
/// moving on to the next line of code

 async fn raw() -> Result<(), reqwest::Error> {
        let coffee = reqwest::Client::new()
    
        .get("https://coffee.alexflipnote.dev/random.json")
        .send()
        .await?
        .text()
        .await?;
    
        println!("{:#?}", coffee);
        Ok(())
    }



/// (3) "formatted" function -                                                                 (3)
/// (lines 2 -9) :
/// import serialize/deserialize from serde to allow parsing of json,
/// create "CoffeeStruct" that represents each item in json string from first function,
/// derive serialize/deserialize traits for the struct so json can be turned into a struct & vice versa

/// (function body):
/// the body of this formatted function had the same steps as last function, 
/// except that the response body will get converted into json instead of a string and use "CoffeeStruct" as a parameter
 
async fn formatted() -> Result<CoffeeStruct, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let body = client
        .get("https://coffee.alexflipnote.dev/random.json")
        .send()
        .await?
        .json::<CoffeeStruct>()
        .await?;


    Ok(body)
}



