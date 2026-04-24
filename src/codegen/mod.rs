mod blocks;
mod preamble;
mod sections;
mod titlepage;

use anyhow::Result;
use crate::ir::Document;
use crate::ir::types::{Block, Section, SectionKind};

pub fn generate(doc: &Document) -> Result<String> {
    let mut out = String::new();

    out.push_str(&preamble::render(doc, has_algorithm(doc))?);
    out.push_str("\n\\begin{document}\n\n");
    out.push_str(&titlepage::render(doc));
    if let Some(text) = &doc.meta.abstract_ {
        out.push_str(&render_abstract(text));
    }
    out.push_str("\\tableofcontents\n\\clearpage\n\n");
    out.push_str(&sections::render_all(doc)?);
    out.push_str("\n\\end{document}\n");

    Ok(out)
}

fn render_abstract(text: &str) -> String {
    format!(
        "\\section*{{Реферат}}\n\\addcontentsline{{toc}}{{section}}{{Реферат}}\n\n{}\n\\clearpage\n\n",
        text.trim()
    )
}

fn has_algorithm(doc: &Document) -> bool {
    doc.document.iter().any(|s| section_has_algorithm(s))
}

fn section_has_algorithm(section: &Section) -> bool {
    match &section.kind {
        SectionKind::Regular { body, subsections } => {
            body.iter().any(block_has_algorithm)
                || subsections.iter().any(section_has_algorithm)
        }
        SectionKind::Bibliography { .. } => false,
    }
}

fn block_has_algorithm(block: &Block) -> bool {
    matches!(block, Block::Algorithm { .. })
}