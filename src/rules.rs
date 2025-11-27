use std::collections::HashSet;

use hex_color::HexColor;

use crate::{CodeTheme, HelixTheme, helix_color::UnderlineStyle};

pub fn write(src: &CodeTheme, dst: &mut HelixTheme) {
    for (key, color) in &src.colors {
        let Some(color) = color else {
            continue;
        };
        let mapped: &[Mapped] = match &**key {
            "editor.background" => &[bg("ui.background")],
            "pickerGroup.border" => &[fg("ui.background.separator"), fg("ui.background")],
            "editorCursor.foreground" => &[bg("ui.cursor")],
            "editorBracketMatch.background" => &[bg("ui.cursor.match")],
            "editorMultiCursor.primary.foreground" => &[bg("ui.cursor.primary")],
            "debugIcon.breakpointForeground" => &[fg("ui.debug.breakpoint")],
            "editorGutter.background" => &[bg("ui.gutter")],
            "editorLineNumber.foreground" => &[fg("ui.linenr")],
            "editorLineNumber.activeForeground" => &[fg("ui.linenr.selected")],
            // use remote cuz its in the left side of vsc status bar
            "statusBar.remoteBackground" => &[bg("ui.statusline")],
            "statusBar.remoteForeground" => &[fg("ui.statusline")],
            "statusBar.background" => &[bg("ui.bufferline"), bg("ui.bufferline.background")],
            "statusBar.foreground" => &[fg("ui.bufferline")],
            "editorHoverWidget.foreground" => &[fg("ui.popup")],
            "editorHoverWidget.background" => &[bg("ui.popup")],
            "foreground" => &[fg("ui.text")],
            "editorGroup.border" => &[fg("ui.window")],
            "quickInput.background" => &[bg("ui.help")],
            "quickInput.foreground" => &[fg("ui.help")],
            "list.activeSelectionBackground" => &[bg("ui.text.focus")],
            "list.activeSelectionForeground" => &[fg("ui.text.focus")],
            "disabledForeground" => &[fg("ui.text.inactive")],
            "editorGhostText.background" => &[bg("ui.text.directory")],
            "editorGhostText.foreground" => &[fg("ui.text.directory")],
            "editorRuler.foreground" => &[fg("ui.virtual.ruler")],
            "editorWhitespace.foreground" => &[fg("ui.virtual.whitespace")],
            // it should be foreground.
            "editorIndentGuide.background" | "editorIndentGuide.background1" => {
                &[fg("ui.virtual.indent-guide")]
            }
            "editorInlayHint.background" => &[bg("ui.virtual.inlay-hint")],
            "editorInlayHint.foreground" => &[fg("ui.virtual.inlay-hint")],
            "editorInlayHint.parameterBackground" => &[bg("ui.virtual.inlay-hint.parameter")],
            "editorInlayHint.parameterForeground" => &[fg("ui.virtual.inlay-hint.parameter")],
            "editorInlayHint.typeBackground" => &[bg("ui.virtual.inlay-hint.type")],
            "editorInlayHint.typeForeground" => &[fg("ui.virtual.inlay-hint.type")],
            "editorSuggestWidget.foreground" => &[fg("ui.menu")],
            "editorSuggestWidget.background" => &[bg("ui.menu")],
            "editorSuggestWidget.selectedForeground" => &[fg("ui.menu.selected")],
            "editorSuggestWidget.selectedBackground" => &[bg("ui.menu.selected")],
            "scrollbar.background" => &[bg("ui.menu.scroll")],
            "scrollbarSlider.background" => &[fg("ui.menu.scroll")],
            "editor.selectionBackground" => &[bg("ui.selection")],
            "editor.selectionForeground" => &[fg("ui.selection")],
            "editor.symbolHighlightBackground" => &[bg("ui.highlight")],
            "editor.stackFrameHighlightBackground" => &[bg("ui.highlight.frameline")],
            "editor.lineHighlightBackground" => &[bg("ui.cursorline.primary")],
            "editorGutter.addedBackground" => &[fg("diff.plus.gutter")],
            "editorGutter.deletedBackground" => &[fg("diff.minus.gutter")],
            "editorGutter.modifiedBackground" => &[fg("diff.delta.gutter")],
            "editorGutter.addedSecondaryBackground" => &[fg("diff.plus")],
            "editorGutter.deletedSecondaryBackground" => &[fg("diff.minus")],
            "editorGutter.modifiedSecondaryBackground" => &[fg("diff.delta")],
            "merge.incomingContentBackground" => &[fg("diff.delta.conflict")],
            "minimap.errorHighlight" => &[fg("error")],
            "minimap.warningHighlight" => &[fg("warning")],
            "minimap.infoHighlight" => &[fg("info"), fg("hint")],
            _ => &[],
        };

        for m in mapped {
            let d = dst
                .colors
                .entry(m.key)
                .or_insert_with(|| Default::default());
            match m.ty {
                MappedTy::Foreground => d.fg = Some(color.clone()),
                MappedTy::Background => d.bg = Some(color.clone()),
            }
        }
    }

    let mut fallback = crate::token_color::Settings::default();
    let mut entity_fallback = crate::token_color::Settings::default();

    for tc in &src.token_colors {
        let Some(scopes) = &tc.scope else {
            fallback = tc.settings.clone();
            continue;
        };
        let scopes: &[Box<str>] = match scopes {
            crate::token_color::Scope::Single(s) => std::slice::from_ref(s),
            crate::token_color::Scope::Multiple(items) => &**items,
        };
        for mut scope in scopes.iter().flat_map(|s| s.split(',')).map(str::trim) {
            const SUFFIXES: &[&str] = &[".rust", ".java", ".groovy", ".css", ".html", ".markdown"];
            for s in SUFFIXES {
                if let Some(s) = scope.strip_suffix(s) {
                    scope = s;
                    break;
                }
            }

            if scope == "entity" {
                entity_fallback = tc.settings.clone();
            }

            let mapped: &[&str] = match scope {
                "entity.other.attribute-name" | "meta.attribute.name" | "meta.attribute" => {
                    &["attribute"]
                }
                "entity.name.type" => &["type", "constructor"],
                "entity.name.type.parameter" => &["type.parameter"],
                "entity.name.type.numeric" | "support.type.primitive" => &["type.builtin"],
                "entity.name.type.enum" => &["type.enum"],
                "variable.other.enummember" => &["type.enum.variant"],
                "constant" => &["constant"],
                "constant.language" => &["constant.builtin"],
                "constant.language.boolean" | "constant.language.bool" => {
                    &["constant.builtin.boolean"]
                }
                "character" | "string.quoted.single.char" => &["constant.character"],
                "constant.numeric" => &["constant.numeric"],
                "constant.numeric.decimal" | "constant.numeric.hex" | "constant.numeric.bin" => {
                    &["constant.numeric.integer"]
                }
                "string" | "string.quoted.double" | "string.quoted" => &["string"],
                "string.regexp" => &["string.regexp"],
                "string.other.link" => &["string.special.url"],
                "comment" => &["comment"],
                "comment.line.double-slash" => &["comment.line"],
                "documentation" | "	comment.line.documentation" => &["comment.line.documentation"],
                "comment.block" => &["comment.block"],
                "comment.block.documentation" => &["comment.block.documentation"],
                "variable" | "entity.name.variable" | "support.variable" => &["variable"],
                "variable.parameter" => &["variable.parameter"],
                "variable.language" => &["variable.builtin"],
                "variable.other" => &["variable.other"],
                "variable.other.property" => &["variable.other.member"],
                "entity.other.attribute-name.class" => &["label"],
                "punctuation" => &["punctuation"],
                "punctuation.comma" | "punctuation.colon" => &["punctuation.delimiter"],
                "punctuation.brackets"
                | "punctuation.brackets.curly"
                | "punctuation.brackets.angle"
                | "punctuation.brackets.attribute"
                | "punctuation.brackets.square" => &["punctuation.bracket"],
                "meta.interpolation" | "punctuation.definition.interpolation" => {
                    &["punctuation.special"]
                }
                "keyword" => &["keyword"],
                "keyword.control" => &["keyword.control"],
                "keyword.other.using" => &["keyword.control.import"],
                "keyword.operator" => &["keyword.operator", "operator"],
                "keyword.directive" => &["keyword.directive"],
                "keyword.other.fn" | "keyword.other.func" => &["keyword.function"],
                "storage" => &["keyword.storage"],
                "storage.type" => &["keyword.storage.type"],
                "storage.modifier" => &["keyword.storage.modifier"],
                "entity.name.function" => &["function"],
                "entity.name.function.preprocessor" => &["function.special", "function.macro"],
                "entity.name.tag" => &["tag"],
                "entity.name.namespace" => &["namespace"],
                "markup.heading" => &["markup.heading"],
                "heading.1" => &["markup.heading.1"],
                "heading.2" => &["markup.heading.2"],
                "heading.3" => &["markup.heading.3"],
                "heading.4" => &["markup.heading.4"],
                "heading.5" => &["markup.heading.5"],
                "heading.6" => &["markup.heading.6"],
                "markup.list.unnumbered" => &["markup.list.unnumbered"],
                "markup.list.numbered" => &["markup.list.numbered"],
                "markup.list.checked" => &["markup.list.checked"],
                "markup.list.unchecked" => &["markup.list.unchecked"],
                "markup.bold" => &["markup.bold"],
                "markup.italic" => &["markup.italic"],
                "markup.strikethrough" => &["markup.strikethrough"],
                "markup.link" => &["markup.link"],
                "markup.link.url" => &["markup.link.url"],
                "markup.link.label" => &["markup.link.label"],
                "markup.link.text" => &["markup.link.text"],
                "markup.quote" => &["markup.quote"],
                "markup.raw" => &["markup.raw"],
                "markup.inline.raw" => &["markup.raw.inline"],
                "markup.block.raw" => &["markup.raw.block"],
                "token.info-token" => &["diagnostic.info", "diagnostic"],
                "token.warn-token" => &["diagnostic.warning"],
                "token.error-token" => &["diagnostic.error"],
                _ => &[],
            };

            for &m in mapped {
                let d = dst.colors.entry(m).or_insert_with(|| Default::default());
                if let Some(fg) = tc.settings.foreground.as_deref() {
                    d.fg = Some(fg.to_owned().into_boxed_str());
                }
                match tc.settings.font_style {
                    crate::token_color::FontStyle::Italic => {
                        d.modifiers
                            .get_or_insert_default()
                            .insert(crate::helix_color::Modifier::Italic);
                    }
                    crate::token_color::FontStyle::Bold => {
                        d.modifiers
                            .get_or_insert_default()
                            .insert(crate::helix_color::Modifier::Bold);
                    }
                    crate::token_color::FontStyle::Strikethrough => {
                        d.modifiers
                            .get_or_insert_default()
                            .insert(crate::helix_color::Modifier::CrossedOut);
                    }
                    crate::token_color::FontStyle::Underline => {
                        d.underline = Some(crate::helix_color::Underline {
                            color: tc.settings.foreground.clone().unwrap_or_default(),
                            style: UnderlineStyle::Line,
                        });
                    }
                    crate::token_color::FontStyle::None => (),
                    crate::token_color::FontStyle::Reset => {
                        d.modifiers = None;
                        d.underline = None;
                    }
                }
            }
        }
    }

    // patch entity for simple themes
    const FALLBACK_ENTITIES: &[&str] = &[
        "type",
        "attribute",
        "function",
        "label",
        "tag",
        "constructor",
    ];
    for &t in FALLBACK_ENTITIES {
        if !dst.colors.contains_key(t) {
            dst.colors.insert(
                t,
                crate::helix_color::Entry {
                    fg: entity_fallback.foreground.clone(),
                    bg: None,
                    underline: if let crate::token_color::FontStyle::Underline =
                        entity_fallback.font_style
                    {
                        Some(crate::helix_color::Underline {
                            color: entity_fallback.foreground.clone().unwrap_or_default(),
                            style: UnderlineStyle::Line,
                        })
                    } else {
                        None
                    },
                    modifiers: match entity_fallback.font_style {
                        crate::token_color::FontStyle::Italic => {
                            Some(crate::helix_color::Modifier::Italic)
                        }
                        crate::token_color::FontStyle::Bold => {
                            Some(crate::helix_color::Modifier::Bold)
                        }
                        crate::token_color::FontStyle::Strikethrough => {
                            Some(crate::helix_color::Modifier::CrossedOut)
                        }
                        _ => None,
                    }
                    .map(|m| HashSet::from_iter([m])),
                },
            );
        }
    }

    // patch stauts line
    if !dst.colors.contains_key("ui.statusline")
        && let Some(bufline) = dst.colors.get("ui.bufferline")
    {
        dst.colors.insert("ui.statusline", bufline.clone());
    }

    // patch cursor
    if let Some(fg) = dst.colors.get("ui.background").and_then(|e| e.bg.clone()) {
        for (_, e) in dst
            .colors
            .iter_mut()
            .filter(|(k, e)| e.fg.is_none() && matches!(**k, "ui.cursor" | "ui.cursor.primary"))
        {
            e.fg = Some(fg.clone());
        }
    }

    // patch diagnostic, following vsc behavior
    const DIAGNOSTIC_NAMES: &[&str] = &[
        "diagnostic",
        "diagnostic.hint",
        "diagnostic.info",
        "diagnostic.warning",
        "diagnostic.error",
        "diagnostic.unnecessary",
        "diagnostic.deprecated",
    ];

    let warning_squiggle = src
        .colors
        .get("editorWarning.foreground")
        .and_then(Option::as_ref);
    let error_squiggle = src
        .colors
        .get("editorError.foreground")
        .and_then(Option::as_ref);
    let info_squiggle = src
        .colors
        .get("editorInfo.foreground")
        .and_then(Option::as_ref);
    let hint_squiggle = src
        .colors
        .get("editorHint.foreground")
        .and_then(Option::as_ref);

    for &n in DIAGNOSTIC_NAMES {
        let squiggle = match n {
            "diagnostic.hint" => hint_squiggle,
            "diagnostic.info" => info_squiggle,
            "diagnostic.warning" => warning_squiggle,
            "diagnositc.error" => error_squiggle,
            _ => None,
        };
        if let Some(e) = dst.colors.get_mut(n)
            && e.underline.is_none()
            && let Some(fg) = e.fg.as_ref().or(squiggle).cloned()
        {
            e.underline = Some(crate::helix_color::Underline {
                color: fg,
                style: UnderlineStyle::Curl,
            })
        }
    }

    // deprecated patch
    dst.colors
        .entry("diagnostic.deprecated")
        .or_default()
        .modifiers
        .get_or_insert_default()
        .insert(crate::helix_color::Modifier::CrossedOut);

    // unused patch
    dst.colors
        .entry("diagnostic.unnecessary")
        .or_default()
        .modifiers
        .get_or_insert_default()
        .insert(crate::helix_color::Modifier::Dim);

    let fallback_bg = dst
        .colors
        .get("ui.background")
        .and_then(|e| e.bg.as_deref())
        .and_then(|bg| HexColor::parse(bg).ok())
        .filter(|color| color.a == u8::MAX);

    for e in dst.colors.values_mut() {
        // final fallback
        if e.fg.is_none() {
            e.fg = fallback.foreground.clone();
        };
        if let Some(u) = &mut e.underline
            && u.color.is_empty()
        {
            u.color = fallback.foreground.clone().unwrap_or_default();
        }

        // alpha handling
        if let Some(bg) = e.bg.as_deref().and_then(|bg| HexColor::parse(bg).ok())
            && bg.a != u8::MAX
            && let Some(fbg) = fallback_bg
        {
            e.bg = Some(
                mask_color(bg, fbg, bg.a)
                    .display_rgb()
                    .to_string()
                    .into_boxed_str(),
            );
        }
        if let Some(fg) = e.fg.as_deref().and_then(|fg| HexColor::parse(fg).ok())
            && fg.a != u8::MAX
            && let Some(bg) =
                e.bg.as_deref()
                    .and_then(|bg| HexColor::parse(bg).ok())
                    .or(fallback_bg)
        {
            e.fg = Some(
                mask_color(fg, bg, fg.a)
                    .display_rgb()
                    .to_string()
                    .into_boxed_str(),
            );
        }
    }
}

fn mask_color(fg: HexColor, bg: HexColor, alpha: u8) -> HexColor {
    let fgc = alpha as f32 / u8::MAX as f32;
    let bgc = 1.0 - fgc;
    fg.scale(fgc).wrapping_add(bg.scale(bgc)).with_a(u8::MAX)
}

struct Mapped {
    key: &'static str,
    ty: MappedTy,
}

enum MappedTy {
    Foreground,
    Background,
}

#[inline]
const fn fg(key: &'static str) -> Mapped {
    Mapped {
        key,
        ty: MappedTy::Foreground,
    }
}

#[inline]
const fn bg(key: &'static str) -> Mapped {
    Mapped {
        key,
        ty: MappedTy::Background,
    }
}
