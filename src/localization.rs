use egui::Context;
use egui_l10n::{ContextExt as _, Localization};

/// Extension methods for [`Context`]
pub(crate) trait ContextExt {
    fn set_localizations(&self);
}

impl ContextExt for Context {
    fn set_localizations(&self) {
        self.set_localization(
            locales::EN,
            Localization::new(locales::EN)
                .with_sources(l10n::EN)
                .with_sources(fatty_acid_expressions::l10n::EN)
                .with_sources(fatty_acid_names::l10n::EN)
                .with_sources(widgets::l10n::EN),
        );
        self.set_localization(
            locales::RU,
            Localization::new(locales::RU)
                .with_sources(l10n::RU)
                .with_sources(fatty_acid_expressions::l10n::RU)
                .with_sources(fatty_acid_names::l10n::RU)
                .with_sources(widgets::l10n::RU),
        );
        self.set_language_identifier(locales::EN)
    }
}

mod locales {
    use egui_l10n::{LanguageIdentifier, langid};

    pub(super) const EN: LanguageIdentifier = langid!("en");
    pub(super) const RU: LanguageIdentifier = langid!("ru");
}

mod l10n {
    use crate::asset;

    pub(super) const EN: &[&str] = &[
        // asset!("/ftl/en/aocs.org.ftl"),
        // asset!("/ftl/en/aocs.org.ext.ftl"),
        asset!("/ftl/en/main.ftl"),
        asset!("/ftl/en/main.ext.ftl"),
        asset!("/ftl/en/properties.ftl"),
    ];

    pub(super) const RU: &[&str] = &[
        // asset!("/ftl/en/aocs.org.ftl"),
        // asset!("/ftl/en/aocs.org.ext.ftl"),
        // asset!("/ftl/ru/main.ftl"),
    ];
}
