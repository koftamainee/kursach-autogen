use anyhow::Result;
use crate::ir::{Document, types::{Section, SectionKind}};

pub fn render_all(doc: &Document) -> Result<String> {
    let mut out = String::new();
    for section in &doc.document {
        out.push_str(&render_section(section, 0)?);
    }
    Ok(out)
}

fn render_section(section: &Section, depth: usize) -> Result<String> {
    let mut out = String::new();

    let cmd = match (depth, section.numbered) {
        (0, true)  => "\\section",
        (0, false) => "\\section*",
        (1, true)  => "\\subsection",
        (1, false) => "\\subsection*",
        (_, true)  => "\\subsubsection",
        _          => "\\subsubsection*",
    };

    let toc_level = match depth {
        0 => "section",
        1 => "subsection",
        _ => "subsubsection",
    };


    match &section.kind {
        SectionKind::Bibliography { .. } => {  }
        SectionKind::Regular { .. } => {
            out.push_str(&format!("{}{{{}}}\n", cmd, section.title));
        }
    }

    if !section.numbered {
        out.push_str(&format!(
            "\\addcontentsline{{toc}}{{{}}}{{{}}}\n",
            toc_level, section.title,
        ));
    }

    out.push('\n');

    match &section.kind {
        SectionKind::Regular { body, subsections } => {
            for block in body {
                out.push_str(&super::blocks::render(block)?);
            }
            for sub in subsections {
                out.push_str(&render_section(sub, depth + 1)?);
            }
        }
        SectionKind::Bibliography { entries } => {
            out.push_str(&render_bibliography(entries, &super::blocks::BibFormat::default()));
        }
    }

    Ok(out)
}

fn render_bibliography(entries: &[crate::ir::types::BibEntry], _fmt: &super::blocks::BibFormat) -> String {
    let mut out = String::new();
    out.push_str(&format!("\\begin{{thebibliography}}{{{}}}\n", entries.len()));

    for entry in entries {
        let text = format_bib_entry(entry);
        out.push_str(&format!("  \\bibitem{{{}}}\n  {}\n", entry.id, text));
    }

    out.push_str("\\end{thebibliography}\n\n");
    out
}

fn format_bib_entry(entry: &crate::ir::types::BibEntry) -> String {
    use crate::ir::types::BibKind;

    match &entry.kind {
        BibKind::Book { authors, title, publisher, year, city, pages } => {
            let authors_str = authors.join(", ");
            let pgs = pages.map(|p| format!(" {} с.", p)).unwrap_or_default();
            format!("{} {}. --- {}: {}, {}.{}",
                    authors_str, title, city, publisher, year, pgs)
        }
        BibKind::Article { authors, title, journal, year, volume, pages } => {
            let authors_str = authors.join(", ");
            let vol = volume.as_deref().map(|v| format!(" Т. {}.", v)).unwrap_or_default();
            let pgs = pages.as_deref().map(|p| format!(" С. {}.", p)).unwrap_or_default();
            format!("{} {} // {}.{} {}.{}", authors_str, title, journal, vol, year, pgs)
        }
        BibKind::Online { authors, title, url, accessed } => {
            let authors_str = if authors.is_empty() {
                String::new()
            } else {
                format!("{} ", authors.join(", "))
            };
            format!(
                "{}{}. --- URL: \\url{{{}}} (дата обращения: {}).",
                authors_str, title, url, accessed,
            )
        }
        BibKind::Thesis { authors, title, degree, city, year, pages } => {
            let authors_str = authors.join(", ");
            let deg = degree.as_deref().unwrap_or("дис. ... канд. техн. наук");
            let pgs = pages.map(|p| format!(" {} с.", p)).unwrap_or_default();
            format!("{} {}: {}. --- {}, {}.{}",
                    authors_str, title, deg, city, year, pgs)
        }
    }
}