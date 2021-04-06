use serde::Deserialize;

use super::{ChangeBasicInfoNoteCommandModel, ChangeParentOfNoteCommandModel, CreateNoteCommandModel};

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) enum CommandModel {
    CreateNoteCommandModel(CreateNoteCommandModel),
    ChangeParentOfNoteCommandModel(ChangeParentOfNoteCommandModel),
    ChangeBasicInfoNoteCommandModel(ChangeBasicInfoNoteCommandModel),
}

impl CommandModel {}
