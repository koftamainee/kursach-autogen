use anyhow::{Context, Result, bail};
use std::collections::{HashSet, HashMap};
use std::path::{Path, PathBuf};

use super::types::{Block, Document, ListItem, Section, SectionKind};

pub fn load(path: &Path) -> Result<Document> {
    let canonical = path
        .canonicalize()
        .with_context(|| format!("Cannot resolve path: {}", path.display()))?;

    let mut visited = HashSet::new();
    let content = resolve_file(&canonical, &mut visited)
        .with_context(|| format!("Failed to load {}", path.display()))?;

    let doc: Document = serde_yaml::from_str(&content)
        .with_context(|| format!("Failed to parse {}", path.display()))?;

    let doc_dir = path.parent().unwrap_or(Path::new("."));
    validate(&doc, doc_dir)?;

    Ok(doc)
}

fn validate(doc: &Document, base_dir: &Path) -> Result<()> {
    let mut ids: HashMap<String, &'static str> = HashMap::new();
    let mut errors: Vec<String> = Vec::new();

    for section in &doc.document {
        collect_section(section, base_dir, &mut ids, &mut errors);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        bail!("Document validation failed:\n{}", errors.join("\n"))
    }
}

fn collect_section(
    section: &Section,
    base_dir: &Path,
    ids: &mut HashMap<String, &'static str>,
    errors: &mut Vec<String>,
) {
    check_id(&section.id, "section", ids, errors);

    match &section.kind {
        SectionKind::Regular { body, subsections } => {
            for block in body {
                collect_block(block, base_dir, ids, errors);
            }
            for sub in subsections {
                collect_section(sub, base_dir, ids, errors);
            }
        }
        SectionKind::Bibliography { entries } => {
            for entry in entries {
                check_id(&entry.id, "bib entry", ids, errors);
            }
        }
    }
}

fn collect_block(
    block: &Block,
    base_dir: &Path,
    ids: &mut HashMap<String, &'static str>,
    errors: &mut Vec<String>,
) {
    match block {
        Block::Formula { id: Some(id), .. } => {
            check_id(id, "formula", ids, errors);
        }
        Block::Figure { id, path, .. } => {
            check_id(id, "figure", ids, errors);
            check_path(path, base_dir, errors);
        }
        Block::FigureGroup { id, figures, .. } => {
            check_id(id, "figure_group", ids, errors);
            for fig in figures {
                check_path(&fig.path, base_dir, errors);
            }
        }
        Block::Listing { id, source, .. } => {
            use super::types::ListingSource;
            check_id(id, "listing", ids, errors);
            if let ListingSource::File { path } = source {
                check_path(path, base_dir, errors);
            }
        }
        Block::Table { id, .. } => {
            check_id(id, "table", ids, errors);
        }
        Block::List { items, .. } => {
            for item in items {
                if let ListItem::Nested(nested) = item {
                    collect_block(nested, base_dir, ids, errors);
                }
            }
        }
        _ => {}
    }
}

fn check_id(
    id: &str,
    kind: &'static str,
    ids: &mut HashMap<String, &'static str>,
    errors: &mut Vec<String>,
) {
    if let Some(prev_kind) = ids.get(id) {
        errors.push(format!(
            "  Duplicate id '{}': first seen as {}, redefined as {}",
            id, prev_kind, kind
        ));
    } else {
        ids.insert(id.to_string(), kind);
    }
}

fn check_path(path: &Path, base_dir: &Path, errors: &mut Vec<String>) {
    let full = base_dir.join(path);
    if !full.exists() {
        errors.push(format!("  Missing file: {}", full.display()));
    }
}

fn resolve_file(path: &PathBuf, visited: &mut HashSet<PathBuf>) -> Result<String> {
    if !visited.insert(path.clone()) {
        let cycle: Vec<_> = visited.iter().map(|p| p.display().to_string()).collect();
        bail!(
            "Cyclic import detected: '{}' is already being imported.\nImport stack: {}",
            path.display(),
            cycle.join(" -> ")
        );
    }

    let base_dir = path.parent().unwrap_or(Path::new("."));
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Cannot read {}", path.display()))?;

    let result = resolve_imports(&content, base_dir, visited)?;

    visited.remove(path);

    Ok(result)
}

fn resolve_imports(content: &str, base_dir: &Path, visited: &mut HashSet<PathBuf>) -> Result<String> {
    let mut result = String::with_capacity(content.len());

    for line in content.lines() {
        match parse_import_tag(line) {
            ImportTag::None => {
                result.push_str(line);
                result.push('\n');
            }
            ImportTag::Bare { path: import_path, indent } => {
                let full_path = base_dir.join(import_path).canonicalize()
                    .with_context(|| format!("Cannot resolve import path: {}/{}", base_dir.display(), import_path))?;
                let imported = resolve_file(&full_path, visited)
                    .with_context(|| format!("Failed to resolve import: {}", full_path.display()))?;

                let mut lines = imported.lines();

                if let Some(first) = lines.next() {
                    result.push_str(indent);
                    result.push_str("- ");
                    result.push_str(first);
                    result.push('\n');

                    let child_indent = format!("{}  ", indent);
                    for rest_line in lines {
                        result.push_str(&child_indent);
                        result.push_str(rest_line);
                        result.push('\n');
                    }
                }
            }
            ImportTag::Keyed { key, path: import_path, indent } => {
                let full_path = base_dir.join(import_path).canonicalize()
                    .with_context(|| format!("Cannot resolve import path: {}/{}", base_dir.display(), import_path))?;
                let imported = resolve_file(&full_path, visited)
                    .with_context(|| format!("Failed to resolve import: {}", full_path.display()))?;

                result.push_str(indent);
                result.push_str(key);
                result.push_str(":\n");

                let child_indent = format!("{}  ", indent);
                for imported_line in imported.lines() {
                    result.push_str(&child_indent);
                    result.push_str(imported_line);
                    result.push('\n');
                }
            }
        }
    }

    Ok(result)
}

enum ImportTag<'a> {
    None,
    Bare { path: &'a str, indent: &'a str },
    Keyed { key: &'a str, path: &'a str, indent: &'a str },
}

fn parse_import_tag(line: &str) -> ImportTag {
    let indent = leading_spaces(line);
    let trimmed = line.trim();

    if let Some(rest) = trimmed.strip_prefix("- !import ") {
        let path = rest.trim().trim_matches('"').trim_matches('\'');
        return ImportTag::Bare { path, indent };
    }

    if let Some(rest) = trimmed.strip_prefix("!import ") {
        let path = rest.trim().trim_matches('"').trim_matches('\'');
        return ImportTag::Bare { path, indent };
    }

    if let Some((key, rest)) = trimmed.split_once(": !import ") {
        let path = rest.trim().trim_matches('"').trim_matches('\'');
        return ImportTag::Keyed { key, path, indent };
    }

    ImportTag::None
}

fn leading_spaces(s: &str) -> &str {
    &s[..s.len() - s.trim_start().len()]
}