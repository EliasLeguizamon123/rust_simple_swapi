use serde::{Deserialize};
use reqwest::{Error};

#[derive(Deserialize, Debug)]
struct Person {
    name: String,
}
#[derive(Deserialize, Debug)]
struct PeopleResponse {
    results: Vec<Person>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let url = "https://swapi.dev/api/people/";

    let response = reqwest::get(url).await?;
    let people : Vec<Person> = response.json::<PeopleResponse>().await?.results;

    println!("People in page 1");
    for person in people {
        println!("Name: {}", person.name);
    }

    Ok(())
}