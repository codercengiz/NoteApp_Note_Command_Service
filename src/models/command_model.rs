use serde::Deserialize;

use super::{ChangeParentOfNoteCommandModel, CreateNoteCommandModel};

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) enum CommandModel {
    CreateNoteCommandModel(CreateNoteCommandModel),
    ChangeParentOfNoteCommandModel(ChangeParentOfNoteCommandModel),
}

impl CommandModel {}
