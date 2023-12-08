// This file was generated by the `theme_importer`.
// Be careful when modifying it by hand.

use gpui::rgba;

#[allow(unused)]
use crate::{
    Appearance, StatusColorsRefinement, ThemeColorsRefinement, UserFontStyle, UserFontWeight,
    UserHighlightStyle, UserSyntaxTheme, UserTheme, UserThemeFamily, UserThemeStylesRefinement,
};

pub fn andromeda() -> UserThemeFamily {
    UserThemeFamily {
        name: "Andromeda".into(),
        author: "Eliver Lara (EliverLara)".into(),
        themes: vec![
            UserTheme {
                name: "Andromeda".into(),
                appearance: Appearance::Dark,
                styles: UserThemeStylesRefinement {
                    colors: ThemeColorsRefinement {
                        border: Some(rgba(0x1b1d23ff).into()),
                        border_variant: Some(rgba(0x1b1d23ff).into()),
                        border_focused: Some(rgba(0x746f77ff).into()),
                        border_selected: Some(rgba(0x1b1d23ff).into()),
                        border_transparent: Some(rgba(0x1b1d23ff).into()),
                        border_disabled: Some(rgba(0x1b1d23ff).into()),
                        elevated_surface_background: Some(rgba(0x23262eff).into()),
                        surface_background: Some(rgba(0x23262eff).into()),
                        background: Some(rgba(0x23262eff).into()),
                        element_background: Some(rgba(0x00e8c5cc).into()),
                        element_hover: Some(rgba(0x23262eff).into()),
                        element_selected: Some(rgba(0x23262eff).into()),
                        drop_target_background: Some(rgba(0x3a404eff).into()),
                        ghost_element_hover: Some(rgba(0x23262eff).into()),
                        text: Some(rgba(0xd5ced9ff).into()),
                        status_bar_background: Some(rgba(0x23262eff).into()),
                        title_bar_background: Some(rgba(0x23262eff).into()),
                        toolbar_background: Some(rgba(0x23262eff).into()),
                        tab_bar_background: Some(rgba(0x23262eff).into()),
                        tab_inactive_background: Some(rgba(0x23262eff).into()),
                        tab_active_background: Some(rgba(0x23262eff).into()),
                        editor_background: Some(rgba(0x23262eff).into()),
                        editor_gutter_background: Some(rgba(0x23262eff).into()),
                        editor_line_number: Some(rgba(0x746f77ff).into()),
                        editor_active_line_number: Some(rgba(0xd5ced9ff).into()),
                        terminal_ansi_bright_red: Some(rgba(0xee5d43ff).into()),
                        terminal_ansi_bright_green: Some(rgba(0x96e072ff).into()),
                        terminal_ansi_bright_yellow: Some(rgba(0xffe66dff).into()),
                        terminal_ansi_bright_blue: Some(rgba(0x7cb7ffff).into()),
                        terminal_ansi_bright_magenta: Some(rgba(0xff00aaff).into()),
                        terminal_ansi_bright_cyan: Some(rgba(0x00e8c6ff).into()),
                        terminal_ansi_red: Some(rgba(0xee5d43ff).into()),
                        terminal_ansi_green: Some(rgba(0x96e072ff).into()),
                        terminal_ansi_yellow: Some(rgba(0xffe66dff).into()),
                        terminal_ansi_blue: Some(rgba(0x7cb7ffff).into()),
                        terminal_ansi_magenta: Some(rgba(0xff00aaff).into()),
                        terminal_ansi_cyan: Some(rgba(0x00e8c6ff).into()),
                        ..Default::default()
                    },
                    status: StatusColorsRefinement {
                        deleted: Some(rgba(0xfc644dff).into()),
                        error: Some(rgba(0xfc644dff).into()),
                        hidden: Some(rgba(0x746f77ff).into()),
                        hint: Some(rgba(0x969696ff).into()),
                        ignored: Some(rgba(0x555555ff).into()),
                        ..Default::default()
                    },
                    syntax: Some(UserSyntaxTheme {
                        highlights: vec![
                            (
                                "attribute".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xffe66dff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "comment".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xa0a1a7cc).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "comment.doc".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xa0a1a7cc).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "constant".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xee5d43ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "constructor".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xf92672ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "emphasis".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xc74dedff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "emphasis.strong".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xf39c12ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "function".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xffe66dff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "keyword".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xc74dedff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "number".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xf39c12ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "operator".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xee5d43ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "string".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x96e072ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "string.escape".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x96e072ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "string.regex".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x96e072ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "string.special".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x96e072ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "string.special.symbol".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x96e072ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "tag".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xf92672ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "text.literal".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x96e072ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "type".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xee5d43ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "variable".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x00e8c6ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "variable.special".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x00e8c6ff).into()),
                                    ..Default::default()
                                },
                            ),
                        ],
                    }),
                },
            },
            UserTheme {
                name: "Andromeda Bordered".into(),
                appearance: Appearance::Dark,
                styles: UserThemeStylesRefinement {
                    colors: ThemeColorsRefinement {
                        border: Some(rgba(0x1b1d23ff).into()),
                        border_variant: Some(rgba(0x1b1d23ff).into()),
                        border_focused: Some(rgba(0x746f77ff).into()),
                        border_selected: Some(rgba(0x1b1d23ff).into()),
                        border_transparent: Some(rgba(0x1b1d23ff).into()),
                        border_disabled: Some(rgba(0x1b1d23ff).into()),
                        elevated_surface_background: Some(rgba(0x23262eff).into()),
                        surface_background: Some(rgba(0x23262eff).into()),
                        background: Some(rgba(0x262a33ff).into()),
                        element_background: Some(rgba(0x00e8c5cc).into()),
                        element_hover: Some(rgba(0x23262eff).into()),
                        element_selected: Some(rgba(0x23262eff).into()),
                        drop_target_background: Some(rgba(0x3a404eff).into()),
                        ghost_element_hover: Some(rgba(0x23262eff).into()),
                        text: Some(rgba(0xd5ced9ff).into()),
                        status_bar_background: Some(rgba(0x23262eff).into()),
                        title_bar_background: Some(rgba(0x23262eff).into()),
                        toolbar_background: Some(rgba(0x262a33ff).into()),
                        tab_bar_background: Some(rgba(0x23262eff).into()),
                        tab_inactive_background: Some(rgba(0x23262eff).into()),
                        tab_active_background: Some(rgba(0x262a33ff).into()),
                        editor_background: Some(rgba(0x262a33ff).into()),
                        editor_gutter_background: Some(rgba(0x262a33ff).into()),
                        editor_line_number: Some(rgba(0x746f77ff).into()),
                        editor_active_line_number: Some(rgba(0xd5ced9ff).into()),
                        terminal_ansi_bright_red: Some(rgba(0xee5d43ff).into()),
                        terminal_ansi_bright_green: Some(rgba(0x96e072ff).into()),
                        terminal_ansi_bright_yellow: Some(rgba(0xffe66dff).into()),
                        terminal_ansi_bright_blue: Some(rgba(0x7cb7ffff).into()),
                        terminal_ansi_bright_magenta: Some(rgba(0xff00aaff).into()),
                        terminal_ansi_bright_cyan: Some(rgba(0x00e8c6ff).into()),
                        terminal_ansi_red: Some(rgba(0xee5d43ff).into()),
                        terminal_ansi_green: Some(rgba(0x96e072ff).into()),
                        terminal_ansi_yellow: Some(rgba(0xffe66dff).into()),
                        terminal_ansi_blue: Some(rgba(0x7cb7ffff).into()),
                        terminal_ansi_magenta: Some(rgba(0xff00aaff).into()),
                        terminal_ansi_cyan: Some(rgba(0x00e8c6ff).into()),
                        ..Default::default()
                    },
                    status: StatusColorsRefinement {
                        deleted: Some(rgba(0xfc644dff).into()),
                        error: Some(rgba(0xfc644dff).into()),
                        hidden: Some(rgba(0x746f77ff).into()),
                        hint: Some(rgba(0x969696ff).into()),
                        ignored: Some(rgba(0x555555ff).into()),
                        ..Default::default()
                    },
                    syntax: Some(UserSyntaxTheme {
                        highlights: vec![
                            (
                                "attribute".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xffe66dff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "comment".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xa0a1a7cc).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "comment.doc".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xa0a1a7cc).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "constant".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xee5d43ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "constructor".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xf92672ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "emphasis".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xc74dedff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "emphasis.strong".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xf39c12ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "function".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xffe66dff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "keyword".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xc74dedff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "number".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xf39c12ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "operator".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xee5d43ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "string".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x96e072ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "string.escape".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x96e072ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "string.regex".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x96e072ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "string.special".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x96e072ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "string.special.symbol".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x96e072ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "tag".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xf92672ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "text.literal".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x96e072ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "type".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xee5d43ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "variable".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x00e8c6ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "variable.special".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x00e8c6ff).into()),
                                    ..Default::default()
                                },
                            ),
                        ],
                    }),
                },
            },
        ],
    }
}
