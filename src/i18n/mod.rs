// SPDX-FileCopyrightText: 2026 Thomas Vincent <thomasvincent@gmail.com>
// SPDX-License-Identifier: MIT

//! Internationalization (i18n) support for Stonktop.
//!
//! Provides compile-time checked translations for 48 locales with POSIX
//! environment detection (`LC_ALL`, `LC_MESSAGES`, `LANG`) and optional
//! override via `--lang` or `STONKTOP_LANG`.

pub mod catalog;

pub use catalog::Catalog;

/// Supported display locales.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Locale {
    /// English (default).
    #[default]
    En,
    /// German.
    De,
    /// Spanish.
    Es,
    /// French.
    Fr,
    /// Italian.
    It,
    /// Brazilian Portuguese.
    PtBr,
    /// Dutch.
    Nl,
    /// Russian.
    Ru,
    /// Ukrainian.
    Uk,
    /// Japanese.
    Ja,
    /// Korean.
    Ko,
    /// Simplified Chinese.
    ZhCn,
    /// Traditional Chinese.
    ZhTw,
    /// Modern Standard Arabic.
    Ar,
    /// Hindi.
    Hi,
    /// Polish.
    Pl,
    /// Turkish.
    Tr,
    /// Swedish.
    Sv,
    /// Vietnamese.
    Vi,
    /// Indonesian.
    Id,
    /// Afrikaans (South Africa).
    AfZa,
    /// Amharic (Ethiopia).
    AmEt,
    /// Egyptian Arabic (Egypt).
    ArEg,
    /// Moroccan Arabic (Morocco).
    ArMa,
    /// Moroccan French (Morocco).
    FrMa,
    /// Senegalese French (Senegal).
    FrSn,
    /// Hausa (Nigeria).
    HaNg,
    /// Igbo (Nigeria).
    IgNg,
    /// Malagasy (Madagascar).
    MgMg,
    /// Kinyarwanda (Rwanda).
    RwRw,
    /// Somali (Somalia).
    SoSo,
    /// Swahili (Kenya).
    SwKe,
    /// isiXhosa (South Africa).
    XhZa,
    /// Yorùbá (Nigeria).
    YoNg,
    /// isiZulu (South Africa).
    ZuZa,
    /// Bengali.
    Bn,
    /// Persian (Farsi).
    Fa,
    /// Hebrew.
    He,
    /// Marathi.
    Mr,
    /// Tamil.
    Ta,
    /// Telugu.
    Te,
    /// Thai.
    Th,
    /// Urdu.
    Ur,
    /// Malay.
    Ms,
    /// Filipino.
    Fil,
    /// Bosnian.
    Bs,
    /// Croatian.
    Hr,
    /// Serbian (Cyrillic).
    Sr,
}

impl Locale {
    /// All 48 supported locales.
    #[allow(dead_code)]
    pub const ALL: [Locale; 48] = [
        Locale::En,
        Locale::De,
        Locale::Es,
        Locale::Fr,
        Locale::It,
        Locale::PtBr,
        Locale::Nl,
        Locale::Ru,
        Locale::Uk,
        Locale::Ja,
        Locale::Ko,
        Locale::ZhCn,
        Locale::ZhTw,
        Locale::Ar,
        Locale::Hi,
        Locale::Pl,
        Locale::Tr,
        Locale::Sv,
        Locale::Vi,
        Locale::Id,
        Locale::AfZa,
        Locale::AmEt,
        Locale::ArEg,
        Locale::ArMa,
        Locale::FrMa,
        Locale::FrSn,
        Locale::HaNg,
        Locale::IgNg,
        Locale::MgMg,
        Locale::RwRw,
        Locale::SoSo,
        Locale::SwKe,
        Locale::XhZa,
        Locale::YoNg,
        Locale::ZuZa,
        Locale::Bn,
        Locale::Fa,
        Locale::He,
        Locale::Mr,
        Locale::Ta,
        Locale::Te,
        Locale::Th,
        Locale::Ur,
        Locale::Ms,
        Locale::Fil,
        Locale::Bs,
        Locale::Hr,
        Locale::Sr,
    ];

    /// Canonical BCP 47 language tag for this locale.
    #[allow(dead_code)]
    pub const fn code(self) -> &'static str {
        match self {
            Locale::En => "en",
            Locale::De => "de",
            Locale::Es => "es",
            Locale::Fr => "fr",
            Locale::It => "it",
            Locale::PtBr => "pt-BR",
            Locale::Nl => "nl",
            Locale::Ru => "ru",
            Locale::Uk => "uk",
            Locale::Ja => "ja",
            Locale::Ko => "ko",
            Locale::ZhCn => "zh-CN",
            Locale::ZhTw => "zh-TW",
            Locale::Ar => "ar",
            Locale::Hi => "hi",
            Locale::Pl => "pl",
            Locale::Tr => "tr",
            Locale::Sv => "sv",
            Locale::Vi => "vi",
            Locale::Id => "id",
            Locale::AfZa => "af-ZA",
            Locale::AmEt => "am-ET",
            Locale::ArEg => "ar-EG",
            Locale::ArMa => "ar-MA",
            Locale::FrMa => "fr-MA",
            Locale::FrSn => "fr-SN",
            Locale::HaNg => "ha-NG",
            Locale::IgNg => "ig-NG",
            Locale::MgMg => "mg-MG",
            Locale::RwRw => "rw-RW",
            Locale::SoSo => "so-SO",
            Locale::SwKe => "sw-KE",
            Locale::XhZa => "xh-ZA",
            Locale::YoNg => "yo-NG",
            Locale::ZuZa => "zu-ZA",
            Locale::Bn => "bn",
            Locale::Fa => "fa",
            Locale::He => "he",
            Locale::Mr => "mr",
            Locale::Ta => "ta",
            Locale::Te => "te",
            Locale::Th => "th",
            Locale::Ur => "ur",
            Locale::Ms => "ms",
            Locale::Fil => "fil",
            Locale::Bs => "bs",
            Locale::Hr => "hr",
            Locale::Sr => "sr",
        }
    }

    /// Return `true` if this locale uses a right-to-left (RTL) script.
    pub const fn is_rtl(self) -> bool {
        matches!(
            self,
            Locale::Ar | Locale::ArEg | Locale::ArMa | Locale::Fa | Locale::He | Locale::Ur
        )
    }

    /// Parse a BCP 47 or POSIX locale tag (for example `de_DE.UTF-8`, `pt-BR`, `zh_Hans_CN`,
    /// `ar_EG.UTF-8`, `fr-MA`, `C`).
    pub fn from_tag(tag: &str) -> Option<Locale> {
        let trimmed = tag.trim();
        if trimmed.is_empty() {
            return None;
        }

        let base = trimmed
            .split('@')
            .next()
            .unwrap_or(trimmed)
            .split('.')
            .next()
            .unwrap_or(trimmed);

        let normalized = base.replace('_', "-").to_ascii_lowercase();

        if normalized == "c" || normalized == "posix" {
            return Some(Locale::En);
        }

        let mut subtags = normalized.split('-');
        let primary = subtags.next().unwrap_or("");
        let mut script: Option<&str> = None;
        let mut region: Option<&str> = None;
        for subtag in subtags {
            if subtag.is_empty() {
                continue;
            }
            // Stop at singleton-led extension or private-use sections (e.g. `u`, `t`, `x`).
            if subtag.len() == 1 {
                break;
            }
            if script.is_none()
                && region.is_none()
                && subtag.len() == 4
                && subtag.chars().all(|c| c.is_ascii_alphabetic())
            {
                script = Some(subtag);
            } else if region.is_none()
                && ((subtag.len() == 2 && subtag.chars().all(|c| c.is_ascii_alphabetic()))
                    || (subtag.len() == 3 && subtag.chars().all(|c| c.is_ascii_digit())))
            {
                region = Some(subtag);
            }
        }

        match primary {
            "en" => Some(Locale::En),
            "de" => Some(Locale::De),
            "es" => Some(Locale::Es),
            "fr" => match region {
                Some("ma") => Some(Locale::FrMa),
                Some("sn") => Some(Locale::FrSn),
                _ => Some(Locale::Fr),
            },
            "it" => Some(Locale::It),
            "pt" => Some(Locale::PtBr),
            "nl" => Some(Locale::Nl),
            "ru" => Some(Locale::Ru),
            "uk" => Some(Locale::Uk),
            "ja" => Some(Locale::Ja),
            "ko" => Some(Locale::Ko),
            "zh" => match script {
                Some("hans") => Some(Locale::ZhCn),
                Some("hant") => Some(Locale::ZhTw),
                _ => match region {
                    Some("tw" | "hk" | "mo") => Some(Locale::ZhTw),
                    _ => Some(Locale::ZhCn),
                },
            },
            "ar" => match region {
                Some("eg") => Some(Locale::ArEg),
                Some("ma") => Some(Locale::ArMa),
                _ => Some(Locale::Ar),
            },
            "hi" => Some(Locale::Hi),
            "pl" => Some(Locale::Pl),
            "tr" => Some(Locale::Tr),
            "sv" => Some(Locale::Sv),
            "vi" => Some(Locale::Vi),
            "id" | "in" => Some(Locale::Id),
            "af" => Some(Locale::AfZa),
            "am" => Some(Locale::AmEt),
            "ha" => Some(Locale::HaNg),
            "ig" => Some(Locale::IgNg),
            "mg" => Some(Locale::MgMg),
            "rw" => Some(Locale::RwRw),
            "so" => Some(Locale::SoSo),
            "sw" => Some(Locale::SwKe),
            "xh" => Some(Locale::XhZa),
            "yo" => Some(Locale::YoNg),
            "zu" => Some(Locale::ZuZa),
            "bn" => Some(Locale::Bn),
            "fa" => Some(Locale::Fa),
            "he" | "iw" => Some(Locale::He),
            "mr" => Some(Locale::Mr),
            "ta" => Some(Locale::Ta),
            "te" => Some(Locale::Te),
            "th" => Some(Locale::Th),
            "ur" => Some(Locale::Ur),
            "ms" => Some(Locale::Ms),
            "fil" | "tl" => Some(Locale::Fil),
            "bs" => Some(Locale::Bs),
            "hr" => Some(Locale::Hr),
            "sr" => Some(Locale::Sr),
            _ => None,
        }
    }

    /// Detect locale from POSIX environment variables (`LC_ALL`, `LC_MESSAGES`, `LANG`).
    pub fn detect() -> Locale {
        Self::detect_from(|var| std::env::var(var).ok())
    }

    fn detect_from<F>(lookup: F) -> Locale
    where
        F: Fn(&str) -> Option<String>,
    {
        for var in ["LC_ALL", "LC_MESSAGES", "LANG"] {
            if let Some(value) = lookup(var) {
                let trimmed = value.trim();
                if trimmed.is_empty() {
                    continue;
                }
                return Self::from_tag(trimmed).unwrap_or(Locale::En);
            }
        }
        Locale::En
    }

    /// Resolve locale from an explicit CLI/env value, falling back to POSIX detection.
    pub fn resolve(explicit: Option<&str>) -> Locale {
        Self::resolve_from(explicit, |var| std::env::var(var).ok())
    }

    fn resolve_from<F>(explicit: Option<&str>, lookup: F) -> Locale
    where
        F: Fn(&str) -> Option<String>,
    {
        if let Some(tag) = explicit {
            return Self::from_tag(tag).unwrap_or(Locale::En);
        }
        if let Some(env_lang) = lookup("STONKTOP_LANG") {
            let trimmed = env_lang.trim();
            if !trimmed.is_empty() {
                return Self::from_tag(trimmed).unwrap_or(Locale::En);
            }
        }
        Self::detect_from(lookup)
    }

    /// Return the static translation catalog for this locale.
    pub const fn catalog(self) -> &'static Catalog {
        catalog::for_locale(self)
    }
}

/// Extract `--lang <value>` or `--lang=<value>` from raw arguments so `--help`
/// output can be localized before full parsing runs.
#[allow(dead_code)]
pub fn extract_cli_lang(args: &[String]) -> Option<String> {
    let mut iter = args.iter().skip(1);
    while let Some(arg) = iter.next() {
        if arg == "--" {
            break;
        }
        if arg == "--lang" {
            return iter.next().cloned();
        }
        if let Some(val) = arg.strip_prefix("--lang=") {
            return Some(val.to_string());
        }
    }
    None
}

/// Extract `--lang <value>` or `--lang=<value>` from raw OS arguments without
/// panicking on non-UTF-8 arguments (such as non-UTF-8 file paths), so `--help`
/// and parse errors can be localized before full parsing runs.
#[allow(dead_code)]
pub fn extract_cli_lang_os(args: &[std::ffi::OsString]) -> Option<String> {
    let mut iter = args.iter().skip(1);
    while let Some(arg) = iter.next() {
        let Some(s) = arg.to_str() else {
            continue;
        };
        if s == "--" {
            break;
        }
        if s == "--lang" {
            return iter
                .next()
                .and_then(|v| v.to_str().map(ToString::to_string));
        }
        if let Some(val) = s.strip_prefix("--lang=") {
            return Some(val.to_string());
        }
    }
    None
}

/// Replace a single `{key}` placeholder in a catalog template.
pub fn fmt1(template: &str, key: &str, value: &str) -> String {
    template.replace(&format!("{{{key}}}"), value)
}

/// Replace three `{key}` placeholders in a catalog template.
pub fn fmt3(
    template: &str,
    key1: &str,
    val1: &str,
    key2: &str,
    val2: &str,
    key3: &str,
    val3: &str,
) -> String {
    template
        .replace(&format!("{{{key1}}}"), val1)
        .replace(&format!("{{{key2}}}"), val2)
        .replace(&format!("{{{key3}}}"), val3)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]
    use super::*;

    fn extract_placeholders(text: &str) -> Vec<String> {
        let mut placeholders = Vec::new();
        let bytes = text.as_bytes();
        let mut i = 0;
        while i < bytes.len() {
            if bytes[i] == b'{' {
                if let Some(end) = text[i + 1..].find('}') {
                    let token = &text[i + 1..i + 1 + end];
                    if !token.is_empty()
                        && token.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
                    {
                        placeholders.push(format!("{{{token}}}"));
                        i += end + 2;
                        continue;
                    }
                }
            }
            i += 1;
        }
        placeholders.sort();
        placeholders
    }

    #[test]
    fn test_all_48_locales_parse() {
        assert_eq!(Locale::ALL.len(), 48);
        for locale in Locale::ALL {
            assert_eq!(Locale::from_tag(locale.code()), Some(locale));
        }
        assert_eq!(Locale::from_tag("C"), Some(Locale::En));
        assert_eq!(Locale::from_tag("POSIX"), Some(Locale::En));
        assert_eq!(Locale::from_tag("en_US.UTF-8"), Some(Locale::En));
        assert_eq!(Locale::from_tag("de_DE.UTF-8@euro"), Some(Locale::De));
        assert_eq!(Locale::from_tag("pt_BR.UTF-8"), Some(Locale::PtBr));
        assert_eq!(Locale::from_tag("zh_Hans_CN.UTF-8"), Some(Locale::ZhCn));
        assert_eq!(Locale::from_tag("zh_TW.UTF-8"), Some(Locale::ZhTw));
        assert_eq!(Locale::from_tag("zh-Hant"), Some(Locale::ZhTw));
        assert_eq!(Locale::from_tag("ar_EG.UTF-8"), Some(Locale::ArEg));
        assert_eq!(Locale::from_tag("ar-Arab-EG"), Some(Locale::ArEg));
        assert_eq!(Locale::from_tag("ar_MA.UTF-8"), Some(Locale::ArMa));
        assert_eq!(Locale::from_tag("ar-Arab-MA"), Some(Locale::ArMa));
        assert_eq!(Locale::from_tag("ar_SA.UTF-8"), Some(Locale::Ar));
        assert_eq!(Locale::from_tag("fr_MA.UTF-8"), Some(Locale::FrMa));
        assert_eq!(Locale::from_tag("fr-Latn-MA"), Some(Locale::FrMa));
        assert_eq!(Locale::from_tag("fr_SN.UTF-8"), Some(Locale::FrSn));
        assert_eq!(Locale::from_tag("fr-Latn-SN"), Some(Locale::FrSn));
        assert_eq!(Locale::from_tag("fr_CA.UTF-8"), Some(Locale::Fr));
        assert_eq!(Locale::from_tag("iw"), Some(Locale::He));
        assert_eq!(Locale::from_tag("iw_IL.UTF-8"), Some(Locale::He));
        assert_eq!(Locale::from_tag("in"), Some(Locale::Id));
        assert_eq!(Locale::from_tag("in_ID.UTF-8"), Some(Locale::Id));
        assert_eq!(Locale::from_tag("tl"), Some(Locale::Fil));
        assert_eq!(Locale::from_tag("tl_PH.UTF-8"), Some(Locale::Fil));
        assert_eq!(Locale::from_tag("bs"), Some(Locale::Bs));
        assert_eq!(Locale::from_tag("bs-BA"), Some(Locale::Bs));
        assert_eq!(Locale::from_tag("bs_BA.UTF-8"), Some(Locale::Bs));
        assert_eq!(Locale::from_tag("hr"), Some(Locale::Hr));
        assert_eq!(Locale::from_tag("hr-HR"), Some(Locale::Hr));
        assert_eq!(Locale::from_tag("hr_HR.UTF-8"), Some(Locale::Hr));
        assert_eq!(Locale::from_tag("sr"), Some(Locale::Sr));
        assert_eq!(Locale::from_tag("sr-RS"), Some(Locale::Sr));
        assert_eq!(Locale::from_tag("sr-Cyrl"), Some(Locale::Sr));
        assert_eq!(Locale::from_tag("sr-Latn"), Some(Locale::Sr));
        assert_eq!(Locale::from_tag("sr-Cyrl-RS"), Some(Locale::Sr));
        assert_eq!(Locale::from_tag("sr-Latn-RS"), Some(Locale::Sr));
        assert_eq!(Locale::from_tag("sr_RS.UTF-8"), Some(Locale::Sr));
        assert_eq!(Locale::from_tag(""), None);
        assert_eq!(Locale::from_tag("zz-INVALID"), None);
    }

    #[test]
    fn test_rtl_locales() {
        let rtl: Vec<Locale> = Locale::ALL.iter().copied().filter(|l| l.is_rtl()).collect();
        let ltr: Vec<Locale> = Locale::ALL
            .iter()
            .copied()
            .filter(|l| !l.is_rtl())
            .collect();
        assert_eq!(rtl.len(), 6);
        assert_eq!(ltr.len(), 42);
        for expected in [
            Locale::Ar,
            Locale::ArEg,
            Locale::ArMa,
            Locale::Fa,
            Locale::He,
            Locale::Ur,
        ] {
            assert!(expected.is_rtl(), "{} should be RTL", expected.code());
        }
    }

    #[test]
    fn test_all_catalog_entries_non_empty_and_placeholder_parity() {
        let en = Locale::En.catalog().entries();
        assert_eq!(en.len(), 146);

        for locale in Locale::ALL {
            let entries = locale.catalog().entries();
            assert_eq!(entries.len(), en.len());
            for ((en_key, en_val), (loc_key, loc_val)) in en.iter().zip(entries.iter()) {
                assert_eq!(en_key, loc_key);
                assert!(
                    !loc_val.trim().is_empty(),
                    "Empty translation for {loc_key} in {}",
                    locale.code()
                );
                assert_eq!(
                    extract_placeholders(loc_val),
                    extract_placeholders(en_val),
                    "Placeholder mismatch for {loc_key} in {}: got {loc_val:?}",
                    locale.code()
                );
            }
        }
    }

    #[test]
    fn test_south_slavic_catalogs_are_distinct() {
        let bs = Locale::Bs.catalog().entries();
        let hr = Locale::Hr.catalog().entries();
        let sr = Locale::Sr.catalog().entries();
        assert_ne!(bs, hr, "Bosnian and Croatian catalogs must be distinct");
        assert_ne!(bs, sr, "Bosnian and Serbian catalogs must be distinct");
        assert_ne!(hr, sr, "Croatian and Serbian catalogs must be distinct");
        assert!(
            Locale::Sr
                .catalog()
                .cli_about
                .chars()
                .any(|c| ('\u{0400}'..='\u{04FF}').contains(&c)),
            "Serbian catalog must use Cyrillic script"
        );
    }

    #[test]
    fn test_default_english_strings_unchanged() {
        let en = Locale::En.catalog();
        assert_eq!(
            en.cli_about,
            "A top-like terminal UI for stock and crypto prices"
        );
        assert_eq!(en.col_symbol, "SYMBOL");
        assert_eq!(en.col_mkt_cap, "MKT CAP");
        assert_eq!(en.err_no_symbols, "Error: No symbols to watch.");
        assert_eq!(en.alert_above, "{symbol} above ${threshold} (${price})");
    }

    #[test]
    fn test_extract_cli_lang() {
        let args = vec![
            "stonktop".to_string(),
            "--lang".to_string(),
            "de".to_string(),
            "--help".to_string(),
        ];
        assert_eq!(extract_cli_lang(&args), Some("de".to_string()));

        let args_eq = vec!["stonktop".to_string(), "--lang=zh-TW".to_string()];
        assert_eq!(extract_cli_lang(&args_eq), Some("zh-TW".to_string()));

        let args_after_sep = vec![
            "stonktop".to_string(),
            "--".to_string(),
            "--lang".to_string(),
            "fr".to_string(),
        ];
        assert_eq!(extract_cli_lang(&args_after_sep), None);

        let args_none = vec!["stonktop".to_string(), "-s".to_string(), "AAPL".to_string()];
        assert_eq!(extract_cli_lang(&args_none), None);
    }

    #[test]
    fn test_bcp47_extensions_and_chinese_script_precedence() {
        // Singleton-led extension / private-use sections must not pollute region/script
        assert_eq!(Locale::from_tag("fr-SN-x-ma"), Some(Locale::FrSn));
        assert_eq!(Locale::from_tag("fr-MA-x-sn"), Some(Locale::FrMa));
        assert_eq!(Locale::from_tag("fr-x-ma"), Some(Locale::Fr));
        assert_eq!(Locale::from_tag("zh-CN-x-tw"), Some(Locale::ZhCn));
        assert_eq!(Locale::from_tag("ar-EG-u-nu-latn"), Some(Locale::ArEg));
        assert_eq!(Locale::from_tag("ar-t-ma"), Some(Locale::Ar));
        assert_eq!(Locale::from_tag("fr-001-x-ma"), Some(Locale::Fr));

        // Chinese explicit script takes precedence over region fallback
        assert_eq!(Locale::from_tag("zh-Hans-HK"), Some(Locale::ZhCn));
        assert_eq!(Locale::from_tag("zh-Hans-TW"), Some(Locale::ZhCn));
        assert_eq!(Locale::from_tag("zh-Hant-CN"), Some(Locale::ZhTw));
        assert_eq!(Locale::from_tag("zh-HK"), Some(Locale::ZhTw));
        assert_eq!(Locale::from_tag("zh-MO"), Some(Locale::ZhTw));
        assert_eq!(Locale::from_tag("zh-CN"), Some(Locale::ZhCn));
        assert_eq!(Locale::from_tag("zh"), Some(Locale::ZhCn));
        assert_eq!(Locale::from_tag("zh--Hans-HK"), Some(Locale::ZhCn));
    }

    #[test]
    fn test_detect_and_resolve_precedence() {
        // LC_ALL > LC_MESSAGES > LANG > En
        assert_eq!(
            Locale::detect_from(|k| match k {
                "LC_ALL" => Some("de_DE.UTF-8".to_string()),
                "LC_MESSAGES" => Some("fr_FR.UTF-8".to_string()),
                "LANG" => Some("ja_JP.UTF-8".to_string()),
                _ => None,
            }),
            Locale::De
        );
        // Empty LC_ALL falls through to LC_MESSAGES
        assert_eq!(
            Locale::detect_from(|k| match k {
                "LC_ALL" => Some("   ".to_string()),
                "LC_MESSAGES" => Some("fr-SN".to_string()),
                "LANG" => Some("ja".to_string()),
                _ => None,
            }),
            Locale::FrSn
        );
        // Falls through to LANG and unknown falls back to En
        assert_eq!(
            Locale::detect_from(|k| match k {
                "LANG" => Some("zh-Hans-HK".to_string()),
                _ => None,
            }),
            Locale::ZhCn
        );
        assert_eq!(
            Locale::detect_from(|k| match k {
                "LANG" => Some("zz-UNKNOWN".to_string()),
                _ => None,
            }),
            Locale::En
        );
        assert_eq!(Locale::detect_from(|_| None), Locale::En);

        // resolve_from: explicit > STONKTOP_LANG > POSIX
        assert_eq!(
            Locale::resolve_from(Some("ja"), |k| match k {
                "STONKTOP_LANG" => Some("de".to_string()),
                _ => None,
            }),
            Locale::Ja
        );
        assert_eq!(
            Locale::resolve_from(Some("invalid-xx"), |_| None),
            Locale::En
        );
        assert_eq!(
            Locale::resolve_from(None, |k| match k {
                "STONKTOP_LANG" => Some("ar-EG".to_string()),
                "LC_ALL" => Some("de".to_string()),
                _ => None,
            }),
            Locale::ArEg
        );
        assert_eq!(
            Locale::resolve_from(None, |k| match k {
                "STONKTOP_LANG" => Some("  ".to_string()),
                "LANG" => Some("ko_KR.UTF-8".to_string()),
                _ => None,
            }),
            Locale::Ko
        );

        // Exercise env-backed wrappers directly without mutating process env
        let _ = Locale::detect();
        assert_eq!(Locale::resolve(Some("de")), Locale::De);
        let _ = Locale::resolve(None);
    }

    #[test]
    fn test_fmt1_and_fmt3() {
        assert_eq!(fmt1("{count} symbols", "count", "5"), "5 symbols");
        assert_eq!(
            fmt3(
                "{symbol} above ${threshold} (${price})",
                "symbol",
                "AAPL",
                "threshold",
                "200.00",
                "price",
                "205.50"
            ),
            "AAPL above $200.00 ($205.50)"
        );
    }

    #[test]
    fn test_extract_cli_lang_os() {
        use std::ffi::OsString;

        let args = vec![
            OsString::from("stonktop"),
            OsString::from("--lang"),
            OsString::from("de"),
            OsString::from("--help"),
        ];
        assert_eq!(extract_cli_lang_os(&args), Some("de".to_string()));

        let args_eq = vec![OsString::from("stonktop"), OsString::from("--lang=fr-MA")];
        assert_eq!(extract_cli_lang_os(&args_eq), Some("fr-MA".to_string()));

        let args_after_sep = vec![
            OsString::from("stonktop"),
            OsString::from("--"),
            OsString::from("--lang"),
            OsString::from("ja"),
        ];
        assert_eq!(extract_cli_lang_os(&args_after_sep), None);

        let args_missing_val = vec![OsString::from("stonktop"), OsString::from("--lang")];
        assert_eq!(extract_cli_lang_os(&args_missing_val), None);

        #[cfg(unix)]
        {
            use std::os::unix::ffi::OsStringExt;
            let bad_utf8 = OsString::from_vec(vec![0xff, 0xfe]);
            let args_with_non_utf8 = vec![
                OsString::from("stonktop"),
                OsString::from("-c"),
                bad_utf8.clone(),
                OsString::from("--lang"),
                OsString::from("de"),
            ];
            assert_eq!(
                extract_cli_lang_os(&args_with_non_utf8),
                Some("de".to_string())
            );

            let args_non_utf8_lang_val = vec![
                OsString::from("stonktop"),
                OsString::from("--lang"),
                bad_utf8,
            ];
            assert_eq!(extract_cli_lang_os(&args_non_utf8_lang_val), None);
        }
    }
}
