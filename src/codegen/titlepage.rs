use crate::ir::Document;

pub fn render(doc: &Document) -> String {
    let m = &doc.meta;

    let logo_block = match &m.logo {
        Some(path) => format!(
            "\\includegraphics[height=2.5cm]{{{}}}\\\\\n",
            path.display()
        ),
        None => String::new(),
    };

    let faculty_block = match &m.faculty {
        Some(f) => format!("\n\n\t\t{}", f),
        None => String::new(),
    };

    let chair_block = match &m.chair {
        Some(c) => format!(
            "\n\n\t\\textbf{{{}}}\n\t\\bigskip\n",
            c
        ),
        None => String::new(),
    };

    let grade_block = if m.grade_line {
        "\\centerline{\\textbf{Оценка}: \\hspace*{8cm} \\textbf{Дата}: \\hspace*{2cm}}\n\
         \\vspace*{1cm}\n"
            .to_string()
    } else {
        String::new()
    };

    let doc_type = m.doc_type.as_deref().unwrap_or("курсовая работа");

    format!(
        r#"\begin{{titlepage}}

\begin{{center}}
	{{\large
{logo}{university}{faculty}

		\bigskip

		{department}
	}}

	\bigskip

{chair}	\vfill \textsc{{\Large {doc_type}}} \\
	{{\large по дисциплине <<{subject}>>}}

	\bigskip

	на тему: {title}
\end{{center}}

\vspace*{{1.5cm}}

\hfill
\begin{{minipage}}{{.6\linewidth}}
	\begin{{tabular}}{{c}}
		\textbf{{Выполнил:}} студент группы {group}    \\\hline \\[.3cm]
		{{\large {author_name}}}                       \\ \hline \scriptsize{{(Фамилия, имя, отчество)}}
		\\[.3cm] \\ \hline
		\scriptsize{{(подпись)}}                       \\[.3cm]
		\textbf{{Принял: }}\hfill {supervisor_title} \\\hline
		\\[.3cm]
		{supervisor_name}                             \\ \hline
		\scriptsize{{(Фамилия, имя, отчество)}}
		\\[.3cm] \\ \hline
		\scriptsize{{(подпись)}}
	\end{{tabular}}
	\vspace*{{1cm}}
\end{{minipage}}

{grade_line}\centerline{{{city}, {year}}}

\end{{titlepage}}
\clearpage
"#,
        logo = logo_block,
        university = m.university,
        faculty = faculty_block,
        department = m.department,
        chair = chair_block,
        doc_type = doc_type,
        subject = m.subject,
        title = m.title,
        group = m.author.group,
        author_name = m.author.name,
        supervisor_title = m.supervisor.title,
        supervisor_name = m.supervisor.name,
        grade_line = grade_block,
        city = m.city,
        year = m.year,
    )
}
