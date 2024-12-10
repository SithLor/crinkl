use reqwest::Error;
use serde::Deserialize;
use std::fs::File;
use std::io::Read;

const BASE_URL: &str = "http://wsdot.wa.gov/Traffic/api/HighwayCameras/HighwayCamerasREST.svc";

#[derive(Deserialize, Debug)]
struct Camera {
    CameraID: i32,
    // Add other fields if needed
}

fn read_api_file(path: &str) -> String {
    let mut file = File::open(path).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error reading file");
    contents
}

async fn find_sr18_highcams(access_code: &str) -> Result<Vec<Camera>, Error> {
    let state_route = "SR-18";
    let region = "";
    let starting_milepost = 0.0;
    let ending_milepost = 100.0; // Adjust as needed

    let url = format!("{}/SearchCamerasAsJson?AccessCode={}&StateRoute={}&Region={}&StartingMilepost={}&EndingMilepost={}", 
                      BASE_URL, access_code, state_route, region, starting_milepost, ending_milepost);
    println!("Fetching SR-18 cameras from URL: {}", url); // Debug statement
    let response = reqwest::get(&url).await?.json::<Vec<Camera>>().await?;
    println!("Received SR-18 cameras response: {:?}", response); // Debug statement
    Ok(response)
}

async fn find_i5_cameras(access_code: &str) -> Result<Vec<Camera>, Error> {
    let state_route = "I-5";
    let region = "";
    let starting_milepost = 0.0;
    let ending_milepost = 1000.0; // Adjust as needed

    let url = format!("{}/SearchCamerasAsJson?AccessCode={}&StateRoute={}&Region={}&StartingMilepost={}&EndingMilepost={}", 
                      BASE_URL, access_code, state_route, region, starting_milepost, ending_milepost);
    println!("Fetching I-5 cameras from URL: {}", url); // Debug statement
    let response = reqwest::get(&url).await?.json::<Vec<Camera>>().await?;
    println!("Received I-5 cameras response: {:?}", response); // Debug statement
    Ok(response)
}

#[tokio::main]
async fn main() {
    let access_code = read_api_file("/workspaces/sprout/perf_db/api.env");
    println!("Access code: {}", access_code); // Debug statement

    match find_sr18_highcams(&access_code).await {
        Ok(cameras) => {
            println!("SR-18 Cameras: {:?}", cameras); // Debug statement
            for camera in cameras {
                println!("SR-18 Camera ID: {}", camera.CameraID);
                // Print other fields if needed
            }
        }
        Err(e) => {
            eprintln!("Error fetching SR-18 cameras: {}", e);
        }
    }

    match find_i5_cameras(&access_code).await {
        Ok(cameras) => {
            println!("I-5 Cameras: {:?}", cameras); // Debug statement
            for camera in cameras {
                println!("I-5 Camera ID: {}", camera.CameraID);
                // Print other fields if needed
            }
        }
        Err(e) => {
            eprintln!("Error fetching I-5 cameras: {}", e);
        }
    }
}