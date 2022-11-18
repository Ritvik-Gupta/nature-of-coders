mod event;
mod model;
mod update;
mod utils;
mod view;

use event::event;
use model::Model;
use update::update;

fn main() {
    nannou::app(Model::for_app)
        .event(event)
        .update(update)
        .fullscreen()
        .run();
}
