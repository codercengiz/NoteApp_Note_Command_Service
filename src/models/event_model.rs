use serde::{Deserialize, Serialize};

use super::{ParentOfNoteChangedEventModel, note_created_event_model::NoteCreatedEventModel};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) enum EventModel {
    NoteCreatedEventModel(NoteCreatedEventModel),
    ParentOfNoteChangedEventModel(ParentOfNoteChangedEventModel),
}
impl EventModel {}
