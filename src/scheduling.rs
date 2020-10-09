/*
// Scheduler, and trait for .seconds(), .minutes(), etc.
use clokwerk::{Scheduler, TimeUnits};
use serde::Deserialize;
use serde_json::json;
use std::env;
use reqwest::Client;

use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};

pub async fn schedule() {
    let mut scheduler = Scheduler::new();
    scheduler.every(30.minutes()).run(|| send2());
}

fn send2() {
    send().await
}

async fn send() {
    println!("schedule");
    // curl -X POST -H "Authorization: Bearer ya29.ElqKBGN2Ri_Uz...HnS_uNreA" -H "Content-Type: application/json" -d '{
    let body = json!({
        "message": {
            "topic" : "location",
            "notification": {
                "body": "This is a Firebase Cloud Messaging Topic Message!",
                "title": "FCM Message"
            }
        }
    });

    let request_url = "https://fcm.googleapis.com/v1/projects/mama-da686/messages:send";
    let response = Client::new()
        .post(request_url)
        .json(&body)
        .header(AUTHORIZATION, "AAAAgDOCCnk:APA91bGKN-1ixblZB6OEQyHRfaGMKnEObRRMxT_haexV1TBNu4OzCRL5XnKrjis8c1DrmoPkfs0k0iU3VHGySfsL84MPbs5SI8Z6C5dMKsIHnCRaXKDGm6_Cplw52ssvyS_o6BWWIf72")
        .header(CONTENT_TYPE, "application/json")
        .send().await;
    
    match response {
        Ok(response) => {
            match response.json::<String>().await {
                Ok(json) => println!("json {:?}", json),
                Err(e) => println!("e {:?}", e),
            } 
        } 
        Err(e) => println!("e {:?}", e),
    };
  //  let gist: fcm = response.json().await?;
  //  println!("Created {:?}", fcm);
}
*/