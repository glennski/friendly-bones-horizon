fn entity_rows(plan: &Plan<'_>, bytes: &[u8]) -> std::ops::ControlFlow<std::result::Result<Vec<u8>, String>> {
    use serde_json::value::RawValue;
    // The content check of this Horizon lists no skeleton among the templates a mod type may copy.
    // Friendly Bones declares its two types with the zombie placeholder and gets the row of its
    // real template here, so the engine builds the types as a skeleton and a wither skeleton.
    let real = |id: &str| match id {
        "friendlybones:friendly_skeleton" => Some("minecraft:skeleton"),
        "friendlybones:friendly_wither_skeleton" => Some("minecraft:wither_skeleton"),
        _ => None,
    };
    if !plan.entity_types.iter().any(|t| real(&t.id).is_some()) {
        return std::ops::ControlFlow::Continue(());
    }
    let build = || -> std::result::Result<Vec<u8>, String> {
        let rows: BTreeMap<String, Box<RawValue>> =
            serde_json::from_slice(bytes).map_err(|e| e.to_string())?;
        let mut out = bytes.trim_ascii_end().to_vec();
        if out.pop() != Some(b'}') {
            return Err("not an object".into());
        }
        let mut empty = rows.is_empty();
        for t in &plan.entity_types {
            if rows.contains_key(&t.id) {
                continue;
            }
            let mut typed = (**t).clone();
            if let Some(template) = real(&t.id) {
                typed.template = template.to_string();
            }
            let Some(row) = rows.get(&typed.template) else {
                continue;
            };
            if !std::mem::take(&mut empty) {
                out.push(b',');
            }
            out.extend(serde_json::to_vec(&typed.id).map_err(|e| e.to_string())?);
            out.push(b':');
            out.extend(mod_entity_row(row, &typed)?);
        }
        out.push(b'}');
        Ok(out)
    };
    std::ops::ControlFlow::Break(build())
}
