mod event;
mod model;
mod utils;
mod view;

use event::event;
use model::Model;

fn main() {
    nannou::app(Model::for_app)
        .event(event)
        .size(500, 500)
        .run();
}
