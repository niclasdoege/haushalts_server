
#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

extern crate serde;
extern crate serde_json;

use std::path::{Path, PathBuf};
use rocket::data::{self, FromData};
use rocket::fs::NamedFile;
use rocket::fs::FileServer;
use rocket::form::Form;
use rocket::http::Status;
use rocket::serde::json::Json;
use serde::de::DeserializeOwned;

use rocket::outcome::Outcome;
use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};

//use rocket_contrib::json::{Json, JsonValue};
use serde::Deserialize;
use serde::Serialize;

use std::fs;
use std::io::prelude::*;
use serde_json::{Value, from_str, to_string_pretty};
use tokio::runtime::Runtime;



/* 
//#[derive(Serialize, Responder)]
#[derive(Responder)]
struct MyResponseData {
    success: bool,
    message: String,
    data: Vec<i32>,
}
*/


#[derive(FromForm)]
struct Input {
    name: String,
    age: i32,
}

#[derive(FromForm)]
struct MyFormData {
    one: String,
}

#[derive(FromForm)]

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Table {
    marc: (String,String),
    mikiya: (String,String),
    niclas: (String,String),
    week: usize
}

#[derive(Clone, PartialEq, Deserialize, Debug, Serialize)]
struct LastDone {
    kitchen: String,
    doorway: String,
    bathroom: String,
    id: usize
}


pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        //response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS, DELETE"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "*"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
        //response.set_
    }
}


use rocket::response::{content, status};

/* 
#[post("/<path>", data = "<form>")]
    fn handle_post_request(path: String, form: Form<MyFormData>) -> content::RawJson<MyResponseData> {
        // Extract the form data from the request
        let form_data = form.into_inner();
        // Do something with the form data and path parameter
        // ...
        // Return a JSON response
        content::RawJson(MyResponseData { success:true, message:"122".to_string(), data:[0,2].to_vec() })
    }
*/
fn update_json_file(week: String, marc: String, mikiya:String, niclas:String) -> Result<(), std::io::Error> {
    // Read the JSON file
    let mut file = fs::File::open("static/example.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Parse the contents of the file into a JSON object
    let mut data: Value = from_str(&contents)?;

    // Update the value of the specified key
    let entry = data
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|entry| entry["week"] == Value:: String(week.clone().into()));
        //println!("{:#?}", from_str(&contents)?);
    if let Some(entry) = entry {
        // Update the value of the specified user
        entry["marc"] = Value::String(marc);
        entry["niclas"] = Value::String(niclas);
        entry["mikiya"] = Value::String(mikiya);
    } else {
        // The specified week was not found in the JSON file
        // You can handle this case as needed
        println!("nononononoononononon");
        println!("{}", week.clone().to_string());
    }

    // Write the modified data back to the file
    let mut file = fs::File::create("static/example.json")?;
    file.write_all(to_string_pretty(&data).unwrap().as_bytes())?;

    Ok(())
}



#[post("/user", data = "<form>")]
    fn json_update_request(form: Form<Table>) {
        // Extract the form data from the request
        let form_data = form.into_inner();

        println!("{}", form_data.week);
        println!("{}", form_data.mikiya.0);
        println!("{}", form_data.marc.0);
        println!("{}", form_data.niclas.0);

        let week = form_data.week;
        let mikiya = form_data.mikiya;
        let marc = form_data.marc;
        let niclas = form_data.niclas;
        // Do something with the form data and path parameter
        // ...
        // Return a JSON response
        
        match update_json_file(week.to_string(), marc.0.to_string(), mikiya.0.to_string(), niclas.0.to_string()) {
            Ok(_) => println!("Successfully updated JSON file"),
            Err(error) => println!("Error updating JSON file: {}", error),
        }
    }

#[post("/", data = "<form>")]
    fn handle_post_request(form: Form<MyFormData>) {
        // Extract the form data from the request
        let form_data = form.into_inner();
        println!("{}", form_data.one);
        println!("{}", form_data.one);
        println!("{}", form_data.one);
        // Do something with the form data and path parameter
        // ...
        // Return a JSON response
    }


#[post("/", data = "<input>")]
    fn new(input: Form<Input>) {  println!("{}", input.name) }

/* 
#[post("/", data = "<task>")]
    fn new(task: Form<Task<'_>>) { /* .. */ }
*/

#[get("/<file..>")]
async fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).await.ok()
}


#[get("/hello/<name>/<age>/<cool>")]
fn hello(name: &str, age: u8, cool: bool) -> String {
    if cool {
        format!("You're a cool {} year old, {}!", age, name)
    } else {
        format!("{}, we need to talk about your coolness.", name)
    }
}

//use rocket::response::status::Status;

#[options("/tables")]
fn options_handler() -> Status {
    Status::Ok
}

#[post("/tables", format = "application/json", data = "<table>")]
    fn add_table(table: Json<Table>) -> Status {
        let table_received = table.into_inner();

        println!("{:#?}", table_received.clone());
        println!("{:#?}", "tables_with_week_received");

        //read local json into string
        let contents = fs::read_to_string("static/example.json").expect("Error reading file");
        //read local json into vector of Table
        let mut tables_from_json: Vec<Table> = serde_json::from_str(&contents).expect("Error parsing JSON");
        //get entry(s) of specified week
        let tables_with_week_from_week_received: Vec<Table> = tables_from_json.clone().into_iter().filter(|table| table.week == table_received.week).collect();
        //remove Table(s) of specified week
        tables_from_json.retain(|x| x.week != table_received.week);
        //update entries for that week


        if tables_with_week_from_week_received.len()>0 {
            let mut table_to_be_manipulated = tables_with_week_from_week_received[0].clone();

            if table_to_be_manipulated.niclas.0.eq("0") {
                table_to_be_manipulated.niclas = table_received.niclas;
            }

            if table_to_be_manipulated.mikiya.0.eq("0") {
                table_to_be_manipulated.mikiya = table_received.mikiya;
            }
            if table_to_be_manipulated.marc.0.eq("0") {
                table_to_be_manipulated.marc = table_received.marc;
            }

            tables_from_json.push(table_to_be_manipulated);

        }
        else {
            let mut table_to_be_manipulated = Table { marc: table_received.marc, mikiya: table_received.mikiya, niclas: table_received.niclas, week: table_received.week };
            tables_from_json.push(table_to_be_manipulated);
        }

            
       
        let new_contents = serde_json::to_string(&tables_from_json).expect("Error serializing tables");
        fs::write("static/example.json", new_contents).expect("Error writing to file");

        change_zustande();

        Status::Ok
    }

fn change_zustande(){

    let contents_zustande = fs::read_to_string("static/zustande.json").expect("Error reading file");
     let mut states_from_json: Vec<LastDone> = serde_json::from_str(&contents_zustande).expect("Error parsing JSON");
     
     let mut highest_id_in_zustande: usize = 0;
     for zustand_in_json in states_from_json.iter(){
         if zustand_in_json.id>highest_id_in_zustande{
             highest_id_in_zustande=zustand_in_json.id;
         }
     }

    //read local json into string
    let contents = fs::read_to_string("static/example.json").expect("Error reading file");
    //read local json into vector of Table
    let mut tables_from_json: Vec<Table> = serde_json::from_str(&contents).expect("Error parsing JSON");

    let mut to_write_as_vec:Vec<LastDone> = vec![];
    let mut id_left_off = highest_id_in_zustande.clone();

    for Table in tables_from_json.iter()  {
        let mut kitchen_last_done="01-01-2023".to_string();
        let mut doorway_last_done="01-01-2023".to_string();
        let mut bathroom_last_done="01-01-2023".to_string();

        if Table.niclas.0.eq("0") ==false{
            match Table.niclas.1.as_str() {
                "kitchen"=>{
                    kitchen_last_done=Table.niclas.0.clone();
                }
                "doorway"=>{
                    doorway_last_done=Table.niclas.0.clone();
                }
                "bathroom"=>{
                    bathroom_last_done=Table.niclas.0.clone();
                }
                &_ => {

                }
            }
        }

        if Table.marc.0.eq("0") ==false{
            match Table.marc.1.as_str() {
                "kitchen"=>{
                    kitchen_last_done=Table.marc.0.clone();
                }
                "doorway"=>{
                    doorway_last_done=Table.marc.0.clone();
                }
                "bathroom"=>{
                    bathroom_last_done=Table.marc.0.clone();
                }
                &_ => {

                }
            }
        }

        if Table.mikiya.0.eq("0") ==false{
            match Table.mikiya.1.as_str() {
                "kitchen"=>{
                    kitchen_last_done=Table.mikiya.0.clone();
                }
                "doorway"=>{
                    doorway_last_done=Table.mikiya.0.clone();
                }
                "bathroom"=>{
                    bathroom_last_done=Table.mikiya.0.clone();
                }
                &_ => {

                }
            }
        }
        
        states_from_json.retain(|x| x.id != Table.week);
        states_from_json.push(LastDone{kitchen:kitchen_last_done, doorway:doorway_last_done, bathroom:bathroom_last_done, id: Table.week})

    }

    let new_contents = serde_json::to_string(&states_from_json).expect("Error serializing tables");
    fs::write("static/zustande.json", new_contents).expect("Error writing to file");


     //change Zustande Json
     
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .attach(CORS)
        .mount("/", routes![options_handler,add_table,hello,handle_post_request,files,json_update_request])
     //   .mount("/static", FileServer::from("static/"))
        .launch()
        .await?;

    Ok(())
}