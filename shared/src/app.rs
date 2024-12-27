use crux_core::render::Render;
use serde::{Deserialize, Serialize};

mod exersizes;
mod sessions;

#[derive(Default, Serialize)]
pub struct Model {
    exersizes: exersizes::Exersizes,
    sessions: sessions::Sessions,
    devInit: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ViewModel {
    pub exersizes: Vec<exersizes::Exersize>,
    pub sessions: Vec<sessions::Session>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum Event {
    //Exersizes
    AddExersize(String),
    AddSession(String),

    //Development
    DevInit(),
}

#[cfg_attr(feature = "typegen", derive(crux_core::macros::Export))]
#[derive(crux_core::macros::Effect)]
pub struct Capabilities {
    pub render: Render<Event>,
}

#[derive(Default)]
pub struct App;

impl crux_core::App for App {
    type Model = Model;
    type Event = Event;
    type ViewModel = ViewModel;
    type Capabilities = Capabilities;

    fn update(&self, msg: Self::Event, model: &mut Self::Model, caps: &Self::Capabilities) {
        match msg {
            Event::AddExersize(name) => {
                model.exersizes.add_exersize(name.clone());
                caps.render.render();
            }
            Event::AddSession(name) => {
                model.sessions.add_session(name.clone());
                caps.render.render();
            }
            Event::DevInit() => {
                if model.devInit {
                    return;
                }
                model.exersizes.add_exersize("Scales".to_string());
                model.sessions.add_session("Jazz Technique".to_string());
                model.devInit = true;
                caps.render.render();
            }
        }
    }

    fn view(&self, model: &Self::Model) -> Self::ViewModel {
        Self::ViewModel {
            exersizes: model.exersizes.value.clone(),
            sessions: model.sessions.value.clone(),
        }
    }
}
