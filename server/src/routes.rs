use actix_web::web;
use crate::controller::user_controller;

pub fn config(cfg: &mut web::ServiceConfig) {
    user_controller::config(cfg);
}