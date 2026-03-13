use tower_lsp::lsp_types::{CompletionItem, CompletionItemKind, Position, Url};

/// Provide completion items for identity.verb field.
/// Returns verb vocabulary completions when the cursor is in the verb position
/// of a semantic address (segment index 2 in a 4-segment L1+ address).
pub fn provide_completions(uri: &Url, _pos: Position) -> Vec<CompletionItem> {
    if !uri.path().ends_with(".manifest.yaml") {
        return vec![];
    }

    // Verb vocabulary from doc 06 (naming-conventions.md)
    // L1 verbs (Query / Command / Event)
    let verbs = [
        ("get", "L1 Query — retrieve without side effects"),
        ("list", "L1 Query — retrieve a collection"),
        ("find", "L1 Query — search by predicate"),
        ("check", "L1 Query — boolean existence check"),
        ("validate", "L1 Query — validate without persisting"),
        ("create", "L1 Command — create a new resource"),
        ("update", "L1 Command — update an existing resource"),
        ("delete", "L1 Command — remove a resource"),
        ("process", "L1 Command — execute a business operation"),
        ("handle", "L1 Command — handle an incoming event/request"),
        ("send", "L1 Command — send a message/notification"),
        ("publish", "L1 Command — publish an event"),
        ("subscribe", "L2 Event subscription"),
        ("stream", "L2 Streaming pipeline"),
        ("aggregate", "L3 Aggregation operation"),
        ("project", "L3 Event sourcing projection"),
        ("compensate", "L3 Saga compensation"),
        ("cache", "L2 Cache aside operation"),
        ("route", "L2 Priority queue routing"),
        ("isolate", "L2 Bulkhead isolation"),
        ("authenticate", "L1 Security — authenticate a principal"),
        ("authorize", "L1 Security — check authorization"),
        ("enrich", "L2 Data enrichment"),
        ("transform", "L1 Data transformation"),
        ("notify", "L1 Notification dispatch"),
    ];

    verbs
        .iter()
        .map(|(verb, detail)| CompletionItem {
            label: verb.to_string(),
            kind: Some(CompletionItemKind::VALUE),
            detail: Some(detail.to_string()),
            ..Default::default()
        })
        .collect()
}
