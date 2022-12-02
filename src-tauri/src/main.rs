#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serde_json::{Value, value};
use serde::{Deserialize, Serialize};

// Index page grab live data button
#[tauri::command]
async fn live_data_fetch() -> Result<String, String> {
    let url = reqwest::get("https://live.alphatiming.co.uk/absolutelymaidenhead.json").await;

    let url_string;

    let mut response_json = String::new();

    match url {
        Ok(e) => {
            if e.status() == reqwest::StatusCode::OK {
                url_string = e.text().await;

                match url_string {
                    Ok(e) => {
                        response_json = String::from(e);
                    }

                    Err(e) => {
                        return Err(e.to_string());
                    }
                }
            } else {
                let sad = "Very sad error code: ".to_string() + &e.status().as_str().to_string();
                return Err(sad);
            }
        }
        Err(e) => {
            return Err(e.to_string());
        }
    }

    let race_info_json: Value = serde_json::from_str(&response_json).unwrap();

    let mut race_info_struct: RaceInfo = serde_json::from_str(&response_json).unwrap();

    let comps = race_info_json["Competitors"].as_array().unwrap().to_vec();

    for i in &comps {
        let competitor_string: String = i.to_string();
        let competitor: CompetitorData = serde_json::from_str(&competitor_string).unwrap();
        race_info_struct.Competitors.push(competitor);
    }

    return Ok(format!(
        "Session Info: {}",
        race_info_struct.Competitors[0].CompetitorName
    ));
}

// Index page read session ID button
#[tauri::command]
async fn session_id_read() -> String {
    let session_json: Value =
        serde_json::from_str(std::fs::read_to_string("sessions.txt").unwrap().as_str()).unwrap();
    let session_json_array = session_json.as_array().unwrap();
    let mut session_id_array: Vec<i32> = Vec::new();

    for i in session_json_array {
        session_id_array.push(i.to_string().parse::<i32>().unwrap());
    }

    format!("{:?}", session_id_array)
}

// Index page update session ids
#[tauri::command]
async fn session_update() -> String {
    let file_session_json: Value =
        serde_json::from_str(std::fs::read_to_string("sessions.txt").unwrap().as_str()).unwrap();

    let file_session_json_array = file_session_json.as_array().unwrap();
    let mut file_session_vec: Vec<i32> = Vec::new();

    for i in file_session_json_array {
        file_session_vec.push(i.to_string().parse::<i32>().unwrap());
    }

    let sessions_response =
        reqwest::get("https://results.alphatiming.co.uk/api/v1/events/absolutelymaidenhead/2022")
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

    let sessions_json: Value = serde_json::from_str(sessions_response.as_str()).unwrap();

    let sessions_array = sessions_json["events"].as_array().unwrap();
    let mut sessions_vec: Vec<i32> = Vec::new();

    for i in sessions_array {
        let session: Value = serde_json::from_value(i.clone()).unwrap();
        sessions_vec.push(
            session["number"]
                .to_string()
                .replace('"', "")
                .parse::<i32>()
                .unwrap(),
        );
    }

    let mut new_session_vec: Vec<i32> = Vec::new();

    for i in sessions_vec {
        let mut new_value = 0;
        for u in file_session_vec.clone() {
            if i == u {
                new_value += 1;
            }
        }

        if new_value == 0 {
            new_session_vec.push(i);
        }
    }

    format!("{:?}", new_session_vec)
}

// Index page download session results
#[tauri::command]
async fn session_detail_download() -> String {

    let all_session_url =
        reqwest::get("https://results.alphatiming.co.uk/api/v1/events/absolutelymaidenhead/2022")
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

    let all_sessions_json: Value = serde_json::from_str(all_session_url.as_str()).unwrap();

    let session_array = all_sessions_json["events"].as_array().unwrap();

    let mut session_details_vec: Vec<Session_Details> = Vec::new();

    for i in session_array {
        let session_number_json: Value = serde_json::from_value(i.clone()).unwrap();
        let session_number = session_number_json["number"]
            .to_string()
            .replace('"', "")
            .parse::<i32>()
            .unwrap();

        let session_info_url = format!(
            "https://results.alphatiming.co.uk/api/v1/absolutelymaidenhead/2022/{}/sessions",
            session_number
        );

        let session_info_response = reqwest::get(session_info_url)
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        let session_info_json: Value =
            serde_json::from_str(session_info_response.as_str()).unwrap();

        let session_info_vec = session_info_json["sessions"].as_array().unwrap();

        

        for s in session_info_vec {
            let session_specs: Value = serde_json::from_value(s.clone()).unwrap();
            let tsid: i32 = session_specs["tsid"].to_string().parse::<i32>().unwrap();
            println!("Session Number: {}, Session TSID: {}", session_number, tsid);

            let session_details_url = format!("https://results.alphatiming.co.uk/api/v1/absolutelymaidenhead/2022/{}/sessions/{}", session_number, tsid);

            let session_details_response = reqwest::get(session_details_url).await.unwrap().text().await.unwrap();

            let session_details_json: Value = serde_json::from_str(session_details_response.as_str()).unwrap();
            let session_results_json: Value = serde_json::from_str(session_details_json["session"].to_string().as_str()).unwrap();
            
            let session_results_competitors_array = session_results_json["competitors"].as_array().unwrap();
            
            let mut session_competitors: Vec<Competitor> = Vec::new();
            
            for r in session_results_competitors_array {
                let competitor_json: Value = serde_json::from_value(r.clone()).unwrap();
                let competitor_name: String = competitor_json["na"].to_string();
                let competitor_grid_pos_option = competitor_json["gp"].to_string().parse::<i8>();
                let mut competitor_grid_pos: i8 = 127;

                match competitor_grid_pos_option {
                    Ok(x) => competitor_grid_pos = x,
                    Err(x) => println!("grid pos null")
                }
                
                let competitor_laptimes_option = competitor_json["laps"].as_array();
                let mut competitor_laptimes_array: &Vec<Value> = &Vec::new();
            
                match competitor_laptimes_option {
                    Some(x) => competitor_laptimes_array = x,
                    None => println!("sad"),
                }
                

                let mut laptimes_vec: Vec<Lap> = Vec::new();

                for l in competitor_laptimes_array {
                    let lapdata: Lap = serde_json::from_value(l.clone()).unwrap();
                    
                    laptimes_vec.push(lapdata);
                }

                let competitor = Competitor {
                    na: competitor_name,
                    gp: competitor_grid_pos,
                    laps: laptimes_vec,
                };

                session_competitors.push(competitor);
            }

            let session = Session_Details {
                id: session_number,
                tsid: tsid,
                start_time: session_specs["startTime"].to_string(),
                event_name: session_info_json["eventName"].to_string(),
                competitors: session_competitors
            };
            println!("Session: {}", session.event_name);
            session_details_vec.push(session);

        }
    }

    println!("{:?}", session_details_vec);

    std::fs::write(
        "sessions_details.json",
        serde_json::to_string(&session_details_vec).unwrap(),
    );

    format!("Saved")

}

//session id reader
#[tauri::command]
async fn read_session_details() -> String {
    let file: String = std::fs::read_to_string("sessions_details.json").unwrap();
    let json: Value = serde_json::from_str(file.as_str()).unwrap();
    let json_array = json.as_array().unwrap();

    let mut struct_vec: Vec<Session_Details> = Vec::new();
    let mut session_id: Vec<i32> = Vec::new();

    for i in json_array {
        let dave = serde_json::to_string(i).unwrap();
        let stru: Session_Details = serde_json::from_str(dave.as_str()).unwrap();
        struct_vec.push(stru);
    }

    for i in struct_vec {
        session_id.push(i.id);
    }
   
    format!("{:?}", session_id)
}

//Graph page tsid retrieve
#[tauri::command]
async fn get_tsid_from_id_struct(idstring: String) -> String {
    let id = idstring.parse::<i32>().unwrap();

    let file: String = std::fs::read_to_string("sessions_details.json").unwrap();
    let json: Value = serde_json::from_str(file.as_str()).unwrap();
    let json_array = json.as_array().unwrap();

    let mut struct_vec: Vec<Session_Details> = Vec::new();

    let mut tsid_vec: Vec<i32> = Vec::new();

    for i in json_array {
        let dave = serde_json::to_string(i).unwrap();
        let stru: Session_Details = serde_json::from_str(dave.as_str()).unwrap();
        struct_vec.push(stru);
    }

    for i in struct_vec {
        if (i.id == id) {
            tsid_vec.push(i.tsid);
        }
    }
   
    format!("{:?}", tsid_vec)
}

// Session page session retriever
#[tauri::command]
async fn session_retrieve(id: i32, tsid: i32) -> String {
    let session_url = format!(
        "https://results.alphatiming.co.uk/api/v1/absolutelymaidenhead/2022/{}/sessions/{}",
        id, tsid
    );

    let session_response = reqwest::get(session_url)
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    format!("{}", session_response)
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            live_data_fetch,
            session_retrieve,
            session_id_read,
            session_update,
            session_detail_download,
            read_session_details,
            get_tsid_from_id_struct,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Serialize, Deserialize, Debug)]
struct RaceInfo {
    SessionStartTime: String,
    EventId: i32,
    SessionId: i32,
    tn: String,
    RaceTime: i32,

    #[serde(skip)]
    Competitors: Vec<CompetitorData>,
}

#[derive(Serialize, Deserialize, Debug)]
struct CompetitorData {
    CompetitorId: i32,
    CompetitorNumber: String,
    CompetitorName: String,
    Position: i32,
    NumberOfLaps: i32,
    LastLaptime: i32,
    RaceTime: i32,
    RunningTime: i32,
    BestLaptime: i32,
    BestLapNumber: i32,
    Gap: String,
    gd: i32,
    Behind: String,
}

#[derive(Serialize, Deserialize)]
struct Sessions {
    session_number: i32,
    session_tsid: Vec<i32>,
}



#[derive(Serialize, Deserialize, Debug)]
struct Session_Details {
    id: i32,
    tsid: i32,
    start_time: String,
    event_name: String,
    competitors: Vec<Competitor>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Competitor {
    na: String,
    laps: Vec<Lap>,
    gp: i8,
}

#[derive(Serialize, Deserialize, Debug)]
struct Lap {
    n: i32,
    lt: i64,
    tt: i64,
}