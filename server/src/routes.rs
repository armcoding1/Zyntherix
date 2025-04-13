use crate::controller::auth_controller;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.configure(auth_controller::config);
}
