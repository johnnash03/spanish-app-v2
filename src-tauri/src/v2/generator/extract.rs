//! Incremental extraction of complete [`GeneratedItem`] objects from a
//! streaming JSON buffer (S5, #36) — v1's streaming-persistence behavior
//! carried over: items are pulled out and validated the moment their
//! closing brace arrives, not when the whole response ends.

use super::types::GeneratedItem;

/// Scans `buffer` for complete `{…}` objects parseable as
/// [`GeneratedItem`]. Returns the items found and the number of bytes
/// consumed from the start of the buffer (the caller drains them and keeps
/// appending stream deltas to the rest).
pub fn extract_complete_items(buffer: &str) -> (Vec<GeneratedItem>, usize) {
    let bytes = buffer.as_bytes();
    let mut items = Vec::new();
    let mut consumed = 0;
    let mut i = 0;

    while i < bytes.len() {
        if bytes[i] != b'{' {
            i += 1;
            continue;
        }
        let start = i;
        let mut depth: i32 = 0;
        let mut in_string = false;
        let mut escape_next = false;
        let mut end = None;

        let mut j = i;
        while j < bytes.len() {
            let c = bytes[j];
            if escape_next {
                escape_next = false;
            } else if in_string {
                match c {
                    b'\\' => escape_next = true,
                    b'"' => in_string = false,
                    _ => {}
                }
            } else {
                match c {
                    b'"' => in_string = true,
                    b'{' => depth += 1,
                    b'}' => {
                        depth -= 1;
                        if depth == 0 {
                            end = Some(j);
                            break;
                        }
                    }
                    _ => {}
                }
            }
            j += 1;
        }

        match end {
            Some(end_idx) => {
                if let Ok(item) =
                    serde_json::from_str::<GeneratedItem>(&buffer[start..=end_idx])
                {
                    items.push(item);
                    consumed = end_idx + 1;
                    i = end_idx + 1;
                } else {
                    i += 1;
                }
            }
            None => break, // Incomplete object — wait for more stream.
        }
    }

    (items, consumed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_a_complete_item_with_variants() {
        let json = r#"[{"slot_id":0,"source":"I want to eat.","canonical":"Quiero comer.","variants":["Yo quiero comer."]}"#;
        let (items, consumed) = extract_complete_items(json);
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].slot_id, 0);
        assert_eq!(items[0].variants, vec!["Yo quiero comer."]);
        assert_eq!(consumed, json.len());
    }

    #[test]
    fn leaves_the_incomplete_tail_in_the_buffer() {
        let partial = r#"[{"slot_id":0,"source":"I want to eat.","canonical":"Quiero comer.","variants":[]},
  {"slot_id":1,"source":"I want to dan"#;
        let (items, consumed) = extract_complete_items(partial);
        assert_eq!(items.len(), 1);
        assert!(consumed < partial.len());
        // After draining, the next delta completes the second item.
        let mut rest = partial[consumed..].to_string();
        rest.push_str(r#"ce.","canonical":"Quiero bailar.","variants":[]}]"#);
        let (more, _) = extract_complete_items(&rest);
        assert_eq!(more.len(), 1);
        assert_eq!(more[0].canonical, "Quiero bailar.");
    }

    #[test]
    fn skips_malformed_objects_without_stalling() {
        let json = r#"[{"nope": true}, {"slot_id":2,"source":"s","canonical":"c","variants":[]}]"#;
        let (items, _) = extract_complete_items(json);
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].slot_id, 2);
    }

    #[test]
    fn empty_buffer_yields_nothing() {
        assert_eq!(extract_complete_items(""), (vec![], 0));
    }
}
