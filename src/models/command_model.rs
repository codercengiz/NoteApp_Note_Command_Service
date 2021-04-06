use serde::Deserialize;

use super::CreateNoteCommandModel;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) enum CommandModel {
    CreateNoteCommandModel(CreateNoteCommandModel),
}

impl CommandModel {}
