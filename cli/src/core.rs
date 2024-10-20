use anyhow::{anyhow, Result};
use crossbeam_channel::Sender;
use std::sync::Arc;
use tracing::debug;

use shared::{App, Effect, Event};

pub type Core = Arc<shared::Core<Effect, App>>;

pub fn new() -> Core {
    Arc::new(shared::Core::new())
}

pub fn update(core: &Core, event: Event, tx: &Arc<Sender<Effect>>) -> Result<()> {
    debug!("event: {:?}", event);

    for effect in core.process_event(event) {
        process_effect(effect, tx)?;
    }
    Ok(())
}

pub fn process_effect(effect: Effect, tx: &Arc<Sender<Effect>>) -> Result<()> {
    debug!("effect: {:?}", effect);

    match effect {
        render @ Effect::Render(_) => {
            tx.send(render).map_err(|e| anyhow!("{:?}", e))?;
        }
    }
    Ok(())
}
