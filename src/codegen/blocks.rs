use anyhow::Result;
use crate::ir::types::Block;

pub fn render(block: &Block) -> Result<String> {
    match block {
        Block::Paragraph { text } => {
            Ok(format!("{}\n\n", process_inline(text)))
        }
        Block::Formula { id, content } => {
            if let Some(id) = id {
                Ok(format!("\\begin{{equation}}\\label{{{}}}\n{}\n\\end{{equation}}\n\n", id, content))
            } else {
                Ok(format!("\\[\n{}\n\\]\n\n", content))
            }
        }
        Block::Figure { id, path, caption, width } => {
            Ok(format!(
                "\\begin{{figure}}[h]\n\\centering\n\\includegraphics[width={}\\linewidth]{{{}}}\n\\caption{{{}}}\\label{{{}}}\n\\end{{figure}}\n\n",
                width,
                path.display(),
                process_inline(caption),
                id,
            ))
        }
        Block::Listing { id, source, language, caption, range } => {
            use crate::ir::types::ListingSource;
            let cap = process_inline(caption);
            match source {
                ListingSource::File { path } => {
                    let range_opts = match range {
                        Some((a, b)) => format!(", firstline={}, lastline={}", a, b),
                        None => String::new(),
                    };
                    Ok(format!(
                        "\\lstinputlisting[language={}, caption={{{}}}, label={}{}]{{{}}}\n\n",
                        language, cap, id, range_opts, path.display(),
                    ))
                }
                ListingSource::Inline { content } => {
                    Ok(format!(
                        "\\begin{{lstlisting}}[language={}, caption={{{}}}, label={}]\n{}\n\\end{{lstlisting}}\n\n",
                        language, cap, id, content.trim_end(),
                    ))
                }
            }
        }
        Block::Table { id, caption, columns, rows } => {
            Ok(render_table(id, caption, columns, rows))
        }
        Block::List { ordered, items } => {
            let env = if *ordered { "enumerate" } else { "itemize" };
            let mut out = format!("\\begin{{{}}}\n", env);
            for item in items {
                match item {
                    crate::ir::types::ListItem::Text(t) => {
                        out.push_str(&format!("  \\item {}\n", process_inline(t)));
                    }
                    crate::ir::types::ListItem::Nested(block) => {
                        out.push_str("  \\item\n");
                        out.push_str(&render(block)?);
                    }
                }
            }
            out.push_str(&format!("\\end{{{}}}\n\n", env));
            Ok(out)
        }
        Block::FigureGroup { id, caption, layout: _, figures } => {
            let mut out = String::from("\\begin{figure}[h]\n\\centering\n");
            for fig in figures {
                out.push_str(&format!(
                    "\\begin{{subfigure}}[b]{{{}\\linewidth}}\n\\includegraphics[width=\\linewidth]{{{}}}\n\\caption{{{}}}\n\\end{{subfigure}}\n\\hfill\n",
                    fig.width,
                    fig.path.display(),
                    process_inline(&fig.subcaption),
                ));
            }
            out.push_str(&format!(
                "\\caption{{{}}}\\label{{{}}}\n\\end{{figure}}\n\n",
                process_inline(caption), id,
            ));
            Ok(out)
        }
        Block::Algorithm { id, caption, numbered, steps } => {
            Ok(render_algorithm(id, caption, *numbered, steps))
        }
        Block::Note { text, title } => {
            let t = title.as_deref().unwrap_or("Примечание");
            Ok(format!(
                "\\begin{{tcolorbox}}[colback=blue!5!white, colframe=blue!50!black, title={}]\n{}\n\\end{{tcolorbox}}\n\n",
                t, process_inline(text)
            ))
        }
        Block::Warning { text, title } => {
            let t = title.as_deref().unwrap_or("Внимание");
            Ok(format!(
                "\\begin{{tcolorbox}}[colback=red!5!white, colframe=red!60!black, title={}]\n{}\n\\end{{tcolorbox}}\n\n",
                t, process_inline(text)
            ))
        }
        Block::PageBreak => Ok("\\clearpage\n\n".to_string()),
        Block::RawLatex { content } => Ok(format!("{}\n\n", content)),
    }
}

pub fn process_inline(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    let mut chars = text.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '{' && chars.peek() == Some(&'{') {
            chars.next();
            let mut marker = String::new();
            let mut closed = false;

            while let Some(c) = chars.next() {
                if c == '}' && chars.peek() == Some(&'}') {
                    chars.next();
                    closed = true;
                    break;
                }
                marker.push(c);
            }

            if closed {
                result.push_str(&render_marker(&marker));
            } else {
                result.push_str("{{");
                result.push_str(&marker);
            }
        } else {
            result.push(ch);
        }
    }

    result
}

fn render_marker(marker: &str) -> String {
    if let Some(id) = marker.strip_prefix("ref:") {
        return format!("\\ref{{{}}}", id);
    }
    if let Some(id) = marker.strip_prefix("pageref:") {
        return format!("\\pageref{{{}}}", id);
    }
    if let Some(id) = marker.strip_prefix("cite:") {
        return format!("\\cite{{{}}}", id);
    }
    if let Some(url) = marker.strip_prefix("url:") {
        return format!("\\url{{{}}}", url);
    }
    if let Some(content) = marker.strip_prefix("bold:") {
        return format!("\\textbf{{{}}}", process_inline(content));
    }
    if let Some(content) = marker.strip_prefix("italic:") {
        return format!("\\textit{{{}}}", process_inline(content));
    }
    if let Some(content) = marker.strip_prefix("code:") {
        return format!("\\texttt{{{}}}", process_inline(content));
    }

    format!("{{{{{}}}}}", marker)
}

fn render_table(id: &str, caption: &str, columns: &[String], rows: &[Vec<String>]) -> String {
    let col_spec = columns.iter().map(|_| "l").collect::<Vec<_>>().join(" | ");
    let header = columns.iter().map(|c| process_inline(c)).collect::<Vec<_>>().join(" & ");

    let mut out = format!(
        "\\begin{{table}}[h]\n\\centering\n\\caption{{{}}}\\label{{{}}}\n\\begin{{tabular}}{{{}}}\n\\hline\n{} \\\\\n\\hline\n",
        process_inline(caption), id, col_spec, header,
    );

    for row in rows {
        let cells = row.iter().map(|c| process_inline(c)).collect::<Vec<_>>().join(" & ");
        out.push_str(&format!("{} \\\\\n", cells));
    }

    out.push_str("\\hline\n\\end{tabular}\n\\end{table}\n\n");
    out
}

#[derive(Default)]
pub struct BibFormat;

fn render_algorithm(id: &str, caption: &str, numbered: bool, steps: &[crate::ir::types::AlgoStep]) -> String {
    let env = if numbered { "algorithm" } else { "algorithm*" };
    let mut out = format!("\\begin{{{}}}\n\\caption{{{}}}\\label{{{}}}\n\\begin{{algorithmic}}[1]\n",
                          env, process_inline(caption), id);
    render_algo_steps(steps, 0, &mut out);
    out.push_str("\\end{algorithmic}\n");
    out.push_str(&format!("\\end{{{}}}\n\n", env));
    out
}

fn render_algo_steps(steps: &[crate::ir::types::AlgoStep], depth: usize, out: &mut String) {
    use crate::ir::types::AlgoStep;
    let indent = "  ".repeat(depth + 1);
    for step in steps {
        match step {
            AlgoStep::Statement { text } => {
                out.push_str(&format!("{}\\State {}\n", indent, process_inline(text)));
            }
            AlgoStep::Require { text } => {
                out.push_str(&format!("{}\\Require {}\n", indent, process_inline(text)));
            }
            AlgoStep::Ensure { text } => {
                out.push_str(&format!("{}\\Ensure {}\n", indent, process_inline(text)));
            }
            AlgoStep::Return { text } => {
                out.push_str(&format!("{}\\State \\Return {}\n", indent, process_inline(text)));
            }
            AlgoStep::Comment { text } => {
                out.push_str(&format!("{}\\Comment{{{}}}\n", indent, process_inline(text)));
            }
            AlgoStep::If { cond, then, else_ } => {
                out.push_str(&format!("{}\\If{{${}$}}\n", indent, cond));
                render_algo_steps(then, depth + 1, out);
                if !else_.is_empty() {
                    out.push_str(&format!("{}\\Else\n", indent));
                    render_algo_steps(else_, depth + 1, out);
                }
                out.push_str(&format!("{}\\EndIf\n", indent));
            }
            AlgoStep::For { var, then } => {
                out.push_str(&format!("{}\\For{{${}$}}\n", indent, var));
                render_algo_steps(then, depth + 1, out);
                out.push_str(&format!("{}\\EndFor\n", indent));
            }
            AlgoStep::While { cond, then } => {
                out.push_str(&format!("{}\\While{{${}$}}\n", indent, cond));
                render_algo_steps(then, depth + 1, out);
                out.push_str(&format!("{}\\EndWhile\n", indent));
            }
        }
    }
}