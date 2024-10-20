use chrono::{serde::ts_milliseconds_option::deserialize as ts_milliseconds_option, DateTime, Utc};
use crux_core::render::Render;
use serde::{Deserialize, Serialize};

// ANCHOR: model
#[derive(Default, Serialize)]
pub struct Model {
    count: Count,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct Count {
    value: isize,
    #[serde(deserialize_with = "ts_milliseconds_option")]
    updated_at: Option<DateTime<Utc>>,
}
// ANCHOR_END: model

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ViewModel {
    pub text: String,
    pub confirmed: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum Event {
    // events from the shell
    Get,
    Increment,
    Decrement,

    // events local to the core
    #[serde(skip)]
    Update(Count),
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
            Event::Get => {
                caps.render.render();
            }
            Event::Update(count) => {
                model.count = count;
                caps.render.render();
            }
            Event::Increment => {
                // optimistic update
                model.count = Count {
                    value: model.count.value + 1,
                    updated_at: None,
                };
                caps.render.render();
            }
            Event::Decrement => {
                // optimistic update
                model.count = Count {
                    value: model.count.value - 1,
                    updated_at: None,
                };
                caps.render.render();
            }
        }
    }

    fn view(&self, model: &Self::Model) -> Self::ViewModel {
        Self::ViewModel {
            text: model.count.value.to_string(),
            confirmed: model.count.updated_at.is_some(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{App, Event, Model};
    use crate::{Count, Effect};
    use chrono::{TimeZone, Utc};
    use crux_core::{assert_effect, testing::AppTester};

    // ANCHOR: simple_tests
    /// Test that a `Get` event does not cause the app to fetch the current
    /// counter value from the web API
    #[test]
    fn get_counter() {
        // instantiate our app via the test harness, which gives us access to the model
        let app = AppTester::<App, _>::default();

        // set up our initial model
        let mut model = Model::default();

        // send a `Get` event to the app
        app.update(Event::Get, &mut model);
    }

    // Test that an `Increment` event causes the app to increment the counter
    #[test]
    fn increment_counter() {
        // instantiate our app via the test harness, which gives us access to the model
        let app = AppTester::<App, _>::default();

        // set up our initial model as though we've previously fetched the counter
        let mut model = Model {
            count: Count {
                value: 1,
                updated_at: Some(Utc.with_ymd_and_hms(2022, 12, 31, 23, 59, 0).unwrap()),
            },
        };

        // send an `Increment` event to the app
        let update = app.update(Event::Increment, &mut model);

        // check that the app asked the shell to render
        assert_effect!(update, Effect::Render(_));

        // we are expecting our model to be updated "optimistically"
        insta::assert_yaml_snapshot!(model, @r###"
        ---
        count:
          value: 2
          updated_at: ~
        "###);
    }

    /// Test that a `Decrement` event causes the app to decrement the counter
    #[test]
    fn decrement_counter() {
        // instantiate our app via the test harness, which gives us access to the model
        let app = AppTester::<App, _>::default();

        // set up our initial model as though we've previously fetched the counter
        let mut model = Model {
            count: Count {
                value: 0,
                updated_at: Some(Utc.with_ymd_and_hms(2022, 12, 31, 23, 59, 0).unwrap()),
            },
        };

        // send a `Decrement` event to the app
        let update = app.update(Event::Decrement, &mut model);

        // check that the app asked the shell to render
        assert_effect!(update, Effect::Render(_));

        // we are expecting our model to be updated "optimistically"
        insta::assert_yaml_snapshot!(model, @r###"
        ---
        count:
          value: -1
          updated_at: ~
        "###);
    }
}
