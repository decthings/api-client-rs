use std::{
    collections::HashMap,
    sync::{Arc, Weak},
};

use crate::{
    rpc::{
        debug::DebugEvent, language::LanguageEvent, spawned::SpawnedEvent, terminal::TerminalEvent,
    },
    StateModification,
};
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub enum DecthingsEvent {
    Debug(DebugEvent),
    Language(LanguageEvent),
    Spawned(SpawnedEvent),
    Terminal(TerminalEvent),
    /// This event will be emitted when the Websocket connection to Decthings closes unexpectedly.
    /// This means that all subscriptions are cancelled. To solve this, call subscribe for the
    /// corresponding API again (no need to call on_event again). Note that you may miss some
    /// events while the subscription is inactive.
    SubscriptionsRemoved,
}

impl DecthingsEvent {
    pub(super) fn deserialize(
        api: &[u8],
        data: &[u8],
        mut additional_segments: Vec<bytes::Bytes>,
    ) -> Result<(DecthingsEvent, StateModification), ()> {
        match api {
            b"Debug" => {
                let mut deserialized: DebugEvent = serde_json::from_slice(data).map_err(|_| ())?;
                let state_modification = match &mut deserialized {
                    DebugEvent::Exit {
                        debug_session_id,
                        reason: _,
                    } => StateModification {
                        add_events: vec![],
                        remove_events: vec![debug_session_id.to_owned()],
                    },
                    DebugEvent::Stdout {
                        debug_session_id: _,
                        data,
                    } => {
                        if additional_segments.is_empty() {
                            return Err(());
                        }
                        *data = additional_segments.remove(0);
                        StateModification::empty()
                    }
                    DebugEvent::Stderr {
                        debug_session_id: _,
                        data,
                    } => {
                        if additional_segments.is_empty() {
                            return Err(());
                        }
                        *data = additional_segments.remove(0);
                        StateModification::empty()
                    }
                    DebugEvent::Initialized {
                        debug_session_id: _,
                    } => StateModification::empty(),
                    DebugEvent::RemoteInspectorData {
                        debug_session_id: _,
                        data,
                    } => {
                        if additional_segments.is_empty() {
                            return Err(());
                        }
                        *data = additional_segments.remove(0);
                        StateModification::empty()
                    }
                };
                Ok((DecthingsEvent::Debug(deserialized), state_modification))
            }
            b"Language" => {
                let mut deserialized: LanguageEvent =
                    serde_json::from_slice(data).map_err(|_| ())?;
                let state_modification = match &mut deserialized {
                    LanguageEvent::Exit {
                        language_server_id,
                        reason: _,
                    } => StateModification {
                        add_events: vec![],
                        remove_events: vec![language_server_id.to_owned()],
                    },
                    LanguageEvent::Data {
                        language_server_id: _,
                        data,
                    } => {
                        if additional_segments.is_empty() {
                            return Err(());
                        }
                        *data = additional_segments.remove(0);
                        StateModification::empty()
                    }
                };
                Ok((DecthingsEvent::Language(deserialized), state_modification))
            }
            b"Spawned" => {
                let mut deserialized: SpawnedEvent =
                    serde_json::from_slice(data).map_err(|_| ())?;
                let state_modification = match &mut deserialized {
                    SpawnedEvent::Exit {
                        spawned_command_id,
                        reason: _,
                    } => StateModification {
                        add_events: vec![],
                        remove_events: vec![spawned_command_id.to_owned()],
                    },
                    SpawnedEvent::Stdout {
                        spawned_command_id: _,
                        data,
                    } => {
                        if additional_segments.is_empty() {
                            return Err(());
                        }
                        *data = additional_segments.remove(0);
                        StateModification::empty()
                    }
                    SpawnedEvent::Stderr {
                        spawned_command_id: _,
                        data,
                    } => {
                        if additional_segments.is_empty() {
                            return Err(());
                        }
                        *data = additional_segments.remove(0);
                        StateModification::empty()
                    }
                };
                Ok((DecthingsEvent::Spawned(deserialized), state_modification))
            }
            b"Terminal" => {
                let mut deserialized: TerminalEvent =
                    serde_json::from_slice(data).map_err(|_| ())?;
                let state_modification = match &mut deserialized {
                    TerminalEvent::Exit {
                        terminal_session_id,
                        reason: _,
                    } => StateModification {
                        add_events: vec![],
                        remove_events: vec![terminal_session_id.to_owned()],
                    },
                    TerminalEvent::Data {
                        terminal_session_id: _,
                        data,
                    } => {
                        if additional_segments.is_empty() {
                            return Err(());
                        }
                        *data = additional_segments.remove(0);
                        StateModification::empty()
                    }
                };
                Ok((DecthingsEvent::Terminal(deserialized), state_modification))
            }
            _ => Err(()),
        }
    }
}

type DecthingsClientEventListener = Box<dyn Fn(&DecthingsEvent) + Send + Sync>;

pub(super) struct EventListeners {
    listeners: Mutex<HashMap<u64, DecthingsClientEventListener>>,
}

impl EventListeners {
    pub fn new() -> Self {
        Self {
            listeners: Mutex::new(HashMap::new()),
        }
    }

    pub async fn add(
        self: &Arc<Self>,
        ev: impl Fn(&DecthingsEvent) + Send + Sync + 'static,
    ) -> EventListenerDisposer {
        let mut lock = self.listeners.lock().await;
        let mut id = 0;
        while lock.contains_key(&id) {
            id += 1;
        }
        lock.insert(id, Box::new(ev));
        drop(lock);
        EventListenerDisposer {
            event_listeners: Arc::downgrade(self),
            id,
        }
    }

    pub async fn call(&self, ev: &DecthingsEvent) {
        let locked = self.listeners.lock().await;
        for listener in locked.values() {
            listener(ev);
        }
    }
}

pub struct EventListenerDisposer {
    event_listeners: Weak<EventListeners>,
    id: u64,
}

impl EventListenerDisposer {
    pub async fn dispose(self) {
        if let Some(event_listeners) = self.event_listeners.upgrade() {
            let mut lock = event_listeners.listeners.lock().await;
            lock.remove(&self.id);
        }
    }
}
