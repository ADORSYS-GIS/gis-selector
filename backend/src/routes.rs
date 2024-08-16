use actix_web::web;

use crate::controllers::applicant_controller::select_applicants;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(select_applicants);
}
