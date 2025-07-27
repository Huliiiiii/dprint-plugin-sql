use dprint_core::configuration::{
  ConfigKeyMap, ConfigKeyValue, GlobalConfiguration, NewLineKind, resolve_global_config,
};

use super::*;

/// Formatting configuration builder.
///
/// # Example
///
/// ```
/// use dprint_plugin_sql::configuration::*;
///
/// let config = ConfigurationBuilder::new()
///     .uppercase(true)
///     .build();
/// ```
pub struct ConfigurationBuilder {
  pub(super) config: ConfigKeyMap,
  global_config: Option<GlobalConfiguration>,
}

impl Default for ConfigurationBuilder {
  fn default() -> Self {
    Self::new()
  }
}

impl ConfigurationBuilder {
  /// Constructs a new configuration builder.
  pub fn new() -> ConfigurationBuilder {
    ConfigurationBuilder {
      config: Default::default(),
      global_config: None,
    }
  }

  /// Gets the final configuration that can be used to format a file.
  pub fn build(&self) -> Configuration {
    if let Some(global_config) = &self.global_config {
      resolve_config(self.config.clone(), global_config).config
    } else {
      let global_config = resolve_global_config(Default::default(), &Default::default()).config;
      resolve_config(self.config.clone(), &global_config).config
    }
  }

  /// Set the global configuration.
  pub fn global_config(&mut self, global_config: GlobalConfiguration) -> &mut Self {
    self.global_config = Some(global_config);
    self
  }

  /// Whether to use tabs (true) or spaces (false).
  ///
  /// Default: `false`
  pub fn use_tabs(&mut self, value: bool) -> &mut Self {
    self.insert("useTabs", value.into())
  }

  /// The number of columns for an indent.
  ///
  /// Default: `4`
  pub fn indent_width(&mut self, value: u8) -> &mut Self {
    self.insert("indentWidth", (value as i32).into())
  }

  /// The kind of newline to use.
  /// Default: `NewLineKind::LineFeed`
  pub fn new_line_kind(&mut self, value: NewLineKind) -> &mut Self {
    self.insert("newLineKind", value.to_string().into())
  }

  /// Use ALL CAPS for reserved words.
  /// Default: `false`
  pub fn uppercase(&mut self, value: bool) -> &mut Self {
    self.insert("uppercase", value.into())
  }

  /// Number of line breaks between queries.
  /// Default: `1`
  pub fn lines_between_queries(&mut self, value: u8) -> &mut Self {
    self.insert("linesBetweenQueries", (value as i32).into())
  }

  /// Whether to format SQL inline (single line) vs multi-line.
  /// Default: `false`
  pub fn inline(&mut self, value: bool) -> &mut Self {
    self.insert("inline", value.into())
  }

  /// Maximum length for inline blocks before breaking to multiple lines.
  /// Default: `50`
  pub fn max_inline_block(&mut self, value: usize) -> &mut Self {
    self.insert("maxInlineBlock", (value as i32).into())
  }

  /// Maximum number of arguments to keep inline.
  /// Default: `None` (no limit)
  pub fn max_inline_arguments(&mut self, value: Option<usize>) -> &mut Self {
    if let Some(v) = value {
      self.insert("maxInlineArguments", (v as i32).into())
    } else {
      self.config.remove("maxInlineArguments");
      self
    }
  }

  /// Maximum length for top-level statements to keep inline.
  /// Default: `None` (no limit)
  pub fn max_inline_top_level(&mut self, value: Option<usize>) -> &mut Self {
    if let Some(v) = value {
      self.insert("maxInlineTopLevel", (v as i32).into())
    } else {
      self.config.remove("maxInlineTopLevel");
      self
    }
  }

  /// Whether to treat JOINs as top-level statements.
  /// Default: `false`
  pub fn joins_as_top_level(&mut self, value: bool) -> &mut Self {
    self.insert("joinsAsTopLevel", value.into())
  }

  #[cfg(test)]
  pub(super) fn get_inner_config(&self) -> ConfigKeyMap {
    self.config.clone()
  }

  fn insert(&mut self, name: &str, value: ConfigKeyValue) -> &mut Self {
    self.config.insert(String::from(name), value);
    self
  }
}

#[cfg(test)]
mod tests {
  use dprint_core::configuration::{NewLineKind, resolve_global_config};

  use super::*;

  #[test]
  fn check_all_values_set() {
    let mut config = ConfigurationBuilder::new();
    config
      .new_line_kind(NewLineKind::CarriageReturnLineFeed)
      .use_tabs(true)
      .indent_width(4)
      .uppercase(true)
      .lines_between_queries(2)
      .inline(true)
      .max_inline_block(100)
      .max_inline_arguments(Some(5))
      .max_inline_top_level(Some(200))
      .joins_as_top_level(true);

    let inner_config = config.get_inner_config();
    assert_eq!(inner_config.len(), 10);
    let diagnostics = resolve_config(
      inner_config,
      &resolve_global_config(Default::default(), &Default::default()).config,
    )
    .diagnostics;
    assert_eq!(diagnostics.len(), 0);
  }

  #[test]
  fn handle_global_config() {
    let mut global_config = ConfigKeyMap::new();
    global_config.insert(String::from("newLineKind"), "crlf".into());
    global_config.insert(String::from("useTabs"), true.into());
    let global_config = resolve_global_config(global_config, &Default::default()).config;
    let mut config_builder = ConfigurationBuilder::new();
    let config = config_builder.global_config(global_config).build();
    assert_eq!(config.new_line_kind == NewLineKind::CarriageReturnLineFeed, true);
    assert_eq!(config.use_tabs, true);
  }

  #[test]
  fn use_defaults_when_global_not_set() {
    let global_config = resolve_global_config(Default::default(), &Default::default()).config;
    let mut config_builder = ConfigurationBuilder::new();
    let config = config_builder.global_config(global_config).build();
    assert_eq!(config.indent_width, 2);
    assert_eq!(config.new_line_kind == NewLineKind::LineFeed, true);
  }
}
