use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct ApplicantCriteria {
    experience: u8,
    education: String,
}

#[derive(Serialize)]
struct SelectedApplicant {
    name: String,
    score: u8,
}

#[post("/select")]
async fn select_applicants(_criteria: web::Json<ApplicantCriteria>) -> impl Responder {

// Placeholder for selection logic
    let selected_applicants = vec![
        SelectedApplicant { name: String::from("John Doe"), score: 85 },
        SelectedApplicant { name: String::from("Jane Smith"), score: 90 },
    ];

    HttpResponse::Ok().json(selected_applicants)
}
