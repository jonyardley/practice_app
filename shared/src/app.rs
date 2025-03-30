use crux_core::{
    render::{render, Render},
    App, Command,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct PracticeGoal {
    pub name: String,
    pub description: String,
    pub status: String,
    pub progress: f64,
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Exercise {
    pub id: u32,
    pub name: String,
}

#[derive(Default)]
pub struct Model {
    goals: Vec<PracticeGoal>,
    exercises: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct ViewModel {
    pub goals: Vec<PracticeGoal>,
    pub exercises: Vec<String>,
}

#[cfg_attr(feature = "typegen", derive(crux_core::macros::Export))]
#[derive(crux_core::macros::Effect)]
#[allow(unused)]
pub struct Capabilities {
    render: Render<Event>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Event {
    GetGoals,
    AddGoal(PracticeGoal),
    GetExercises,
    AddExercise(String),
    SetDevData(),
}

#[derive(Default)]
pub struct Chopin;

impl App for Chopin {
    type Event = Event;
    type Model = Model;
    type ViewModel = ViewModel;
    type Capabilities = Capabilities;
    type Effect = Effect;

    fn update(
        &self,
        event: Self::Event,
        model: &mut Self::Model,
        _caps: &Self::Capabilities,
    ) -> Command<Effect, Event> {
        match event {
            Event::GetGoals => (),
            Event::AddGoal(goal) => {
                let new_goal = PracticeGoal {
                    name: goal.name,
                    description: goal.description,
                    status: goal.status,
                    progress: goal.progress,
                    tags: goal.tags,
                };
                model.goals.push(new_goal)
            }
            Event::GetExercises => (),
            Event::AddExercise(exercise) => model.exercises.push(exercise),
            Event::SetDevData() => {
                model.goals.push(PracticeGoal {
                    name: "Master Nocturnes".to_string(),
                    description: "Play all nocturnes".to_string(),
                    status: "active".to_string(),
                    progress: 0.1,
                    tags: vec!["practice".to_string(), "goal".to_string()],
                });
                model.goals.push(PracticeGoal {
                    name: "Perfect Rachmaninoff Etudes".to_string(),
                    description: "Play all etudes".to_string(),
                    status: "active".to_string(),
                    progress: 0.8,
                    tags: vec!["practice".to_string(), "goal".to_string()],
                });
                model.goals.push(PracticeGoal {
                    name: "Perfect Chopin Etudes".to_string(),
                    description: "Play all etudes".to_string(),
                    status: "completed".to_string(),
                    progress: 0.9,
                    tags: vec!["practice".to_string(), "goal".to_string()],
                });

                model.exercises.push("Scales and Arpeggios".to_string());
                model.exercises.push("Chord Progressions".to_string());
            }
        };

        render()
    }

    fn view(&self, model: &Self::Model) -> Self::ViewModel {
        ViewModel {
            goals: model.goals.clone(),
            exercises: model.exercises.clone(),
        }
    }
}

// *************
// TESTS
// *************
#[cfg(test)]
mod test {
    use super::*;
    use crux_core::{assert_effect, testing::AppTester};

    #[test]
    fn renders() {
        let app = AppTester::<Chopin>::default();
        let mut model = Model::default();

        let update = app.update(Event::GetExercises, &mut model);

        // Check update asked us to `Render`
        assert_effect!(update, Effect::Render(_));
    }

    #[test]
    fn adds_exercise() {
        let app = AppTester::<Chopin>::default();
        let mut model = Model::default();

        let update = app.update(Event::AddExercise("Exercise".to_string()), &mut model);

        // Check update asked us to `Render`
        assert_effect!(update, Effect::Render(_));
    }

    #[test]
    fn adds_goal() {
        let app = AppTester::<Chopin>::default();
        let mut model = Model::default();

        let update = app.update(
            Event::AddGoal(PracticeGoal {
                name: "Goal".to_string(),
                description: "".to_string(),
                status: "".to_string(),
                progress: 0.0,
                tags: vec![],
            }),
            &mut model,
        );

        // Check update asked us to `Render`
        assert_effect!(update, Effect::Render(_));
    }
}
