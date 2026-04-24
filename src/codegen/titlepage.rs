use crate::ir::Document;

pub fn render(doc: &Document) -> String {
    let m = &doc.meta;

    let logo_block = match &m.logo {
        Some(path) => format!(
            "\\includegraphics[height=2.5cm]{{{}}}\\\\\n[0.5cm]\n",
            path.display()
        ),
        None => String::new(),
    };

    format!(
        r#"\begin{{titlepage}}
\centering

{logo}{university}\\[0.2cm]
{department}\\[2cm]

{subject}\\[0.5cm]
\textbf{{\Large {title}}}\\[3cm]

\begin{{flushright}}
\begin{{tabular}}{{l}}
Выполнил: {author_name}\\
Группа: {group}\\[0.5cm]
Руководитель: {supervisor_name}\\
{supervisor_title}\\
\end{{tabular}}
\end{{flushright}}

\vfill
{city} \\cyrdash {year}

\end{{titlepage}}
\clearpage
"#,
        logo = logo_block,
        university = m.university,
        department = m.department,
        subject = m.subject,
        title = m.title,
        author_name = m.author.name,
        group = m.author.group,
        supervisor_name = m.supervisor.name,
        supervisor_title = m.supervisor.title,
        city = m.city,
        year = m.year,
    )
}