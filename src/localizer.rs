// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with
// this file, You can obtain one at https://mozilla.org/MPL/2.0/.

use lang::Lang;
use error::Result;
use macrogen;

/// Main struct for initiating localization for a project.
///
/// # Example
///
/// ```rust
/// use crowbook_localize::Localizer;
/// let fr = r#"
/// msgid "Hello, {}"
/// msgstr "Bonjour, {}"
/// "#;
/// let es = r#"
/// msgid "Hello, {}"
/// msgstr "Hola, {}"
/// "#;
/// let mut localizer = Localizer::new();
/// localizer.add_lang("fr", fr).unwrap();
/// localizer.add_lang("es", es).unwrap();
/// ```
pub struct Localizer {
    langs: Vec<Lang>,
}

impl Localizer {
    /// Create a new, empty Localizer
    pub fn new() -> Localizer {
        Localizer { langs: vec!() }
    }

    /// Add a lang to the localizer
    ///
    /// # Arguments
    ///
    /// * `lang`: the code of the language (e.g. "fr", "en", ...);
    /// * `s`: a string containing localization information. It should be foramtted
    /// similarly to gettext `mo` files.
    pub fn add_lang<S: Into<String>>(&mut self, lang: S, s: &str) -> Result<()> {
        let lang = try!(Lang::new_from_str(lang, s));
        self.langs.push(lang);
        Ok(())
    }

    /// Generate the `localization_macros.rs` file.
    pub fn generate_macro_file(mut self) -> String {
        macrogen::generate_macro_file(&mut self.langs)
    }
}