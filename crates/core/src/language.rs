use crate::meta_var::{extract_meta_var, MetaVariable};
use crate::AstGrep;
use crate::StrDoc;
use std::borrow::Cow;
use std::path::Path;
pub use tree_sitter::Language as TSLanguage;

/// Trait to abstract ts-language usage in ast-grep, which includes:
/// * which character is used for meta variable.
/// * if we need to use other char in meta var for parser at runtime
/// * pre process the Pattern code.
pub trait Language: Clone {
  /// Return the file language from path. Return None if the file type is not supported.
  fn from_path<P: AsRef<Path>>(_path: P) -> Option<Self> {
    // TODO: throw panic here if not implemented properly?
    None
  }

  /// Create an [`AstGrep`] instance for the language
  fn ast_grep<S: AsRef<str>>(&self, source: S) -> AstGrep<StrDoc<Self>> {
    AstGrep::new(source, self.clone())
  }

  /// tree sitter language to parse the source
  fn get_ts_language(&self) -> TSLanguage;
  /// ignore trivial tokens in language matching
  fn skippable_kind_ids(&self) -> &'static [u16] {
    &[]
  }

  /// normalize pattern code before matching
  /// e.g. remove expression_statement, or prefer parsing {} to object over block
  fn pre_process_pattern<'q>(&self, query: &'q str) -> Cow<'q, str> {
    Cow::Borrowed(query)
  }

  /// Configure meta variable special character
  /// By default $ is the metavar char, but in PHP it can be #
  #[inline]
  fn meta_var_char(&self) -> char {
    '$'
  }

  /// Some language does not accept $ as the leading char for identifiers.
  /// We need to change $ to other char at run-time to make parser happy, thus the name expando.
  /// By default this is the same as meta_var char so replacement is done at runtime.
  #[inline]
  fn expando_char(&self) -> char {
    self.meta_var_char()
  }

  /// extract MetaVariable from a given source string
  /// At runtime we need to use expand_char
  fn extract_meta_var(&self, source: &str) -> Option<MetaVariable> {
    extract_meta_var(source, self.expando_char())
  }
}

impl Language for TSLanguage {
  fn get_ts_language(&self) -> TSLanguage {
    self.clone()
  }
}

#[cfg(test)]
mod test {
  use super::*;
  #[derive(Clone)]
  pub struct Tsx;
  impl Language for Tsx {
    fn from_path<P: AsRef<Path>>(_path: P) -> Option<Self> {
      Some(Tsx)
    }
    fn get_ts_language(&self) -> TSLanguage {
      tree_sitter_typescript::language_tsx().into()
    }
  }
}

#[cfg(test)]
pub use test::*;
