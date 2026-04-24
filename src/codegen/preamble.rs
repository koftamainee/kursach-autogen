use anyhow::Result;
use crate::ir::{Document, types::Style};

pub fn render(doc: &Document, use_algorithm: bool) -> Result<String> {
    let s = &doc.style;
    let mut out = String::new();

    out.push_str(&render_documentclass(s));
    out.push_str(&render_geometry(s));
    out.push_str(&render_packages(s, use_algorithm));
    out.push_str(&render_fonts(s));
    out.push_str(&render_listings(s));
    out.push_str(&render_misc(s));

    Ok(out)
}

fn render_documentclass(s: &Style) -> String {
    let size = &s.fonts.main.size;
    format!("\\documentclass[{},a4paper]{{extarticle}}\n\n", size)
}

fn render_geometry(s: &Style) -> String {
    let m = &s.page.margins;
    format!(
        "\\usepackage[left={}, right={}, top={}, bottom={}]{{geometry}}\n",
        m.left, m.right, m.top, m.bottom,
    )
}

fn render_packages(s: &Style, use_algorithm: bool) -> String {
    let mut out = String::new();

    out.push_str("\\usepackage{polyglossia}\n");
    out.push_str("\\setdefaultlanguage{russian}\n");
    out.push_str("\\setotherlanguage{english}\n");
    out.push_str("\\defaultfontfeatures{Mapping=tex-text}\n");
    out.push_str("\\usepackage{graphicx}\n");
    out.push_str("\\usepackage{subcaption}\n");
    out.push_str("\\usepackage{amsmath}\n");
    out.push_str("\\usepackage{amssymb}\n");
    out.push_str("\\usepackage{hyperref}\n");
    out.push_str("\\usepackage{listings}\n");
    out.push_str("\\usepackage{xcolor}\n");
    out.push_str("\\usepackage{caption}\n");
    out.push_str("\\usepackage{setspace}\n");
    out.push_str("\\usepackage{indentfirst}\n");
    out.push_str("\\usepackage{titlesec}\n");
    out.push_str("\\usepackage{tocloft}\n");
    out.push_str("\\usepackage{longtable}\n");
    out.push_str("\\usepackage{booktabs}\n");
    out.push_str("\\usepackage{fancyhdr}\n");
    out.push_str("\\usepackage{tcolorbox}\n");
    if use_algorithm {
        out.push_str("\\usepackage{algorithm}\n");
        out.push_str("\\usepackage{algpseudocode}\n");
    }

    let fig_pos = &s.figures.caption_position;
    let tbl_pos = &s.tables.caption_position;
    out.push_str(&format!(
        "\\captionsetup[figure]{{position={}, labelsep=period, font={{small,it}}}}\n",
        fig_pos,
    ));
    out.push_str(&format!(
        "\\captionsetup[table]{{position={}, labelsep=period, font={{small,it}}}}\n",
        tbl_pos,
    ));

    out.push('\n');
    out
}

fn render_fonts(s: &Style) -> String {
    let mut out = String::new();

    let main = &s.fonts.main;
    let heading = &s.fonts.heading;
    let spacing = main.line_spacing;

    out.push_str("\\usepackage{fontspec}\n");
    out.push_str(&format!("\\setmainfont{{{}}}\n", main.family));
    out.push_str(&format!(
        "\\newfontfamily\\cyrillicfont{{{}}}[Script=Cyrillic]\n",
        main.family,
    ));
    out.push_str(&format!(
        "\\newfontfamily\\cyrillicfontsf{{{}}}[Script=Cyrillic]\n",
        main.family,
    ));
    out.push_str(&format!(
        "\\newfontfamily\\cyrillicfonttt{{{}}}[Script=Cyrillic]\n",
        s.fonts.listing_body.family,
    ));
    out.push_str(&format!("\\setmonofont{{{}}}\n", s.fonts.listing_body.family));

    out.push_str(&format!("\\setstretch{{{}}}\n", spacing));

    let heading_pt = heading.size.trim_end_matches("pt");
    let heading_baseline = (heading_pt.parse::<f32>().unwrap_or(16.0) * heading.line_spacing) as u32;

    let bold_cmd = if heading.bold == Some(true) { "\\bfseries" } else { "" };

    out.push_str(&format!(
        "\\titleformat{{\\section}}{{\\normalfont{}\\fontsize{{{}}}{{{}}}\\selectfont}}{{\\thesection}}{{1em}}{{}}\n",
        bold_cmd, heading_pt, heading_baseline,
    ));
    out.push_str(&format!(
        "\\titleformat{{\\subsection}}{{\\normalfont{}\\fontsize{{{}}}{{{}}}\\selectfont}}{{\\thesubsection}}{{1em}}{{}}\n",
        bold_cmd, heading_pt, heading_baseline,
    ));

    out.push('\n');
    out
}

fn render_listings(s: &Style) -> String {
    let font = &s.fonts.listing_body;
    let size_cmd = pt_to_latex_size(&font.size);

    format!(
        r#"\lstset{{
  basicstyle=\{size}\ttfamily,
  breaklines=true,
  frame=single,
  captionpos=t,
  numbers=left,
  numberstyle=\tiny,
  tabsize=4,
  showstringspaces=false,
  keywordstyle=\color{{blue}},
  commentstyle=\color{{gray}},
  stringstyle=\color{{orange}},
}}

"#,
        size = size_cmd,
    )
}

fn render_misc(s: &Style) -> String {
    let mut out = String::new();

    out.push_str("\\hypersetup{colorlinks=true, linkcolor=black, urlcolor=blue, citecolor=black}\n");

    if s.page.numbering.show_on_title {
        out.push_str("\\pagestyle{plain}\n");
    } else {
        out.push_str("\\pagestyle{fancy}\n");
        out.push_str("\\fancyhf{}\n");
        out.push_str("\\fancyfoot[C]{\\thepage}\n");
        out.push_str("\\renewcommand{\\headrulewidth}{0pt}\n");
    }

    out.push_str("\\setlength{\\parindent}{1.25cm}\n");
    out.push('\n');
    out
}

fn pt_to_latex_size(size: &str) -> &'static str {
    match size.trim_end_matches("pt") {
        "8"  => "tiny",
        "9"  => "footnotesize",
        "10" => "small",
        "11" => "normalsize",
        "12" => "small",
        "14" => "large",
        "16" => "Large",
        "18" => "LARGE",
        "20" => "huge",
        "24" => "Huge",
        _    => "normalsize",
    }
}
