use dprint_core::configuration::NewLineKind;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Configuration {
  pub use_tabs: bool,
  pub indent_width: u8,
  pub new_line_kind: NewLineKind,
  pub uppercase: bool,
  pub lines_between_queries: u8,
  pub inline: bool,
  pub max_inline_block: usize,
  pub max_inline_arguments: Option<usize>,
  pub max_inline_top_level: Option<usize>,
  pub joins_as_top_level: bool,
}
