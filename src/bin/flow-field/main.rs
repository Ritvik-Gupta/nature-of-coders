mod event;
mod model;
mod utils;
mod view;

use event::{event, update};
use model::Model;

fn main() {
    nannou::app(Model::for_app)
        .event(event)
        .update(update)
        .size(600, 600)
        .run();
}
