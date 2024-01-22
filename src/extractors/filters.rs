use super::from_context;

use crate::filters::CommandObject;

from_context!([Client], CommandObject, "command");
