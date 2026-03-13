use tower_lsp::lsp_types::{Hover, HoverContents, MarkupContent, MarkupKind, Position, Url};

/// Provide hover documentation for fields in a .manifest.yaml file.
/// Returns field-level documentation from the ARIA manifest schema.
pub fn provide_hover(uri: &Url, _pos: Position) -> Option<Hover> {
    if !uri.path().ends_with(".manifest.yaml") {
        return None;
    }

    // Field hover documentation (stub — a full implementation would parse the
    // YAML AST to determine which field the cursor is on)
    Some(Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: [
                "## ARIA Manifest Field",
                "",
                "See [ARIA Manifest Schema](https://aria.dev/docs/manifest-schema) for full documentation.",
                "",
                "Run `aria-build check` to validate this manifest.",
            ]
            .join("\n"),
        }),
        range: None,
    })
}
