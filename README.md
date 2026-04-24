# kursach YAML documentation

kursach generates a LaTeX document from a tree of YAML files. The root file declares document metadata, style, and a list of sections. Sections and style can be split across multiple files using `!import`.

---

## Project structure

```
kursach.yaml          ← root file
style/
  gost.yaml           ← style definition
sections/
  intro.yaml          ← a regular section
  references.yaml     ← bibliography section
```

Run the tool:

```
kursach kursach.yaml --output out/
```

---

## Root file

```yaml
version: "1"

meta:
  university: "My University"
  department: "Department of Computer Science"
  subject: "Probability Theory"
  title: "Course Work. Variant 1"
  author:
    name: "Ivan Ivanov"
    group: "CS-21"
  supervisor:
    name: "Pyotr Petrov"
    title: "Associate Professor, PhD"
  year: 2026
  city: "Moscow"
  logo: "img/logo.png"        # optional — path to university logo image
  abstract_: |                # optional — shown after title page, before TOC
    This work studies probabilistic modelling of queuing systems.
    Implemented in C++23 with Qt6. Total length: 30 pages.

style: !import "style/gost.yaml"

document:
  - !import "sections/intro.yaml"
  - !import "sections/references.yaml"
```

All `meta` fields except `logo` and `abstract_` are required.

---

## Imports

`!import "path/to/file.yaml"` inlines another YAML file at that position. Paths are relative to the file containing the import. Circular imports are detected and reported as errors.

The tag can appear in three positions:

```yaml
style: !import "style/gost.yaml"          # keyed — file becomes value of key

document:
  - !import "sections/intro.yaml"         # bare list item
  - !import "sections/references.yaml"
```

---

## Style file

```yaml
page:
  margins:
    left: 20mm
    right: 15mm
    top: 15mm
    bottom: 15mm
  numbering:
    start_from: 1
    show_on_title: false    # if false, page number is hidden on title page

fonts:
  main:
    family: "Times New Roman"
    size: 14pt
    line_spacing: 1.15
    align: justify
    indent: true
  heading:
    family: "Times New Roman"
    size: 16pt
    line_spacing: 1.5
    align: left
    bold: false
  caption:
    family: "Times New Roman"
    size: 12pt
    line_spacing: 1.0
    italic: true
  listing_body:
    family: "DejaVu Sans Mono"
    size: 12pt
    line_spacing: 1.0
    align: left

figures:
  align: center
  caption_position: below
  caption_format: "Figure {n}. {text}"

tables:
  align: center
  caption_position: above
  caption_format: "Table {n}. {text}"

listings:
  caption_position: above
  caption_format: "Listing {n}. {text}"

bibliography:
  standard: GOST-7.0.100-2018
  label_format: "[{n}]"
```

Margin values must be in the form `\d+(mm|cm|pt)`, e.g. `20mm`, `2cm`, `36pt`.

Font sizes supported: `8pt`, `9pt`, `10pt`, `11pt`, `12pt`, `14pt`, `16pt`, `18pt`, `20pt`, `24pt`.

---

## Sections

A section file contains a single section object.

```yaml
id: intro              # unique identifier (required)
title: "Introduction"  # displayed title (required)
numbered: false        # whether section is numbered (default: false)
body:
  - ...                # list of blocks
subsections:
  - ...                # nested sections (same structure, optional)
```

A bibliography section uses `entries` instead of `body`:

```yaml
id: references
title: "References"
numbered: false
entries:
  - ...
```

---

## Inline markers

Inside any `text`, `caption`, or `title` string, you can use `{{marker}}` syntax:

| Marker | Output |
|---|---|
| `{{cite:ref.id}}` | `\cite{ref.id}` — cite a bibliography entry |
| `{{ref:fig.id}}` | `\ref{fig.id}` — reference a labeled element |
| `{{pageref:fig.id}}` | `\pageref{fig.id}` — page number of a label |
| `{{url:https://example.com}}` | `\url{https://example.com}` |
| `{{bold:some text}}` | **some text** |
| `{{italic:some text}}` | *some text* |
| `{{code:some text}}` | `some text` (monospace) |

Markers can be nested: `{{bold:see {{cite:ref.id}}}}`.

---

## Blocks

Every item in `body` has a `type` field that determines its kind.

### paragraph

Plain text paragraph. Inline markers are supported.

```yaml
- type: paragraph
  text: |
    This section describes the method used. See {{cite:ref.knuth}} for details.
```

### formula

A mathematical formula in LaTeX math syntax. If `id` is provided, it is numbered and can be referenced with `{{ref:id}}`.

```yaml
- type: formula
  id: eq.bayes
  content: 'P(A \mid B) = \frac{P(B \mid A)\,P(A)}{P(B)}'

- type: formula
  content: 'E = mc^2'
```

### figure

A single image. `width` is a fraction of `\linewidth` (default: `0.8`).

```yaml
- type: figure
  id: fig.result
  path: "img/result.png"
  caption: "Simulation result"
  width: 0.7
```

### figure_group

Multiple images side by side with individual subcaptions and a shared caption. `layout` is `horizontal` (default) or `vertical`.

```yaml
- type: figure_group
  id: fig.comparison
  caption: "Comparison of methods"
  layout: horizontal
  figures:
    - path: "img/method_a.png"
      subcaption: "Method A"
      width: 0.45
    - path: "img/method_b.png"
      subcaption: "Method B"
      width: 0.45
```

### listing

A code listing. Source can be a file or inline content. `range` limits which lines are included (inclusive, 1-based).

```yaml
- type: listing
  id: lst.main
  language: C++
  caption: "Main simulation loop"
  path: "src/main.cpp"
  range: [10, 40]

- type: listing
  id: lst.formula
  language: Python
  caption: "Bayes calculation"
  content: |
    def bayes(p_a, p_b_given_a, p_b):
        return p_b_given_a * p_a / p_b
```

### table

A table with a header row and data rows.

```yaml
- type: table
  id: tbl.results
  caption: "Experiment results"
  columns: ["Parameter", "Value", "Unit"]
  rows:
    - ["Sample size", "1000", "—"]
    - ["Mean", "3.14", "s"]
    - ["Std dev", "0.27", "s"]
```

### list

An ordered or unordered list. Items can be plain strings or nested blocks.

```yaml
- type: list
  ordered: false
  items:
    - "First item"
    - "Second item"

- type: list
  ordered: true
  items:
    - "Step one"
    - "Step two"
    - type: listing
      id: lst.nested
      language: Bash
      caption: "Run tests"
      content: "cargo test"
```

### algorithm

Structured pseudocode rendered with `algorithmicx`. Steps are nested using `then`.

```yaml
- type: algorithm
  id: alg.search
  caption: "Binary search"
  numbered: true
  steps:
    - kind: require
      text: 'sorted array $A$, target $x$'
    - kind: statement
      text: '$lo \gets 0$, $hi \gets n-1$'
    - kind: while
      cond: 'lo \leq hi'
      then:
        - kind: statement
          text: '$mid \gets \lfloor(lo+hi)/2\rfloor$'
        - kind: if
          cond: 'A[mid] = x'
          then:
            - kind: return
              text: '$mid$'
          else_:
            - kind: if
              cond: 'A[mid] < x'
              then:
                - kind: statement
                  text: '$lo \gets mid + 1$'
              else_:
                - kind: statement
                  text: '$hi \gets mid - 1$'
    - kind: return
      text: '$-1$'
```

Available step kinds: `statement`, `require`, `ensure`, `return`, `comment`, `if`, `for`, `while`.

`if` supports an optional `else_` list. `for` uses `var` for the loop variable expression. `while` uses `cond`.

### note

A callout box with a blue border. Default title is «Примечание». Inline markers are supported in `text`.

```yaml
- type: note
  text: "This feature requires Qt 6.4 or later."

- type: note
  title: "Tip"
  text: "Use a fixed {{code:seed}} value during debugging for reproducible results."
```

### warning

A callout box with a red border. Default title is «Внимание».

```yaml
- type: warning
  text: "Running without initializing the RNG will produce incorrect results."

- type: warning
  title: "Breaking change"
  text: "The config format changed in version 2. Old files are not compatible."
```

### page_break

Inserts a hard page break.

```yaml
- type: page_break
```

### raw_latex

Inserts raw LaTeX verbatim. Use as a last resort when no other block covers your need.

```yaml
- type: raw_latex
  content: '\vspace{2cm}'
```

---

## Bibliography section

The bibliography section uses `entries` instead of `body`. Each entry has an `id` used with `{{cite:id}}` and a `type` that determines its format.

### book

```yaml
- id: ref.knuth
  type: book
  authors: ["Knuth D.E."]
  title: "The Art of Computer Programming, Vol. 1"
  publisher: "Addison-Wesley"
  city: "Reading"
  year: 1997
  pages: 672        # optional
```

### article

```yaml
- id: ref.shannon
  type: article
  authors: ["Shannon C.E."]
  title: "A Mathematical Theory of Communication"
  journal: "Bell System Technical Journal"
  year: 1948
  volume: "27"      # optional
  pages: "379–423"  # optional
```

### online

```yaml
- id: ref.qt
  type: online
  authors: []       # may be empty
  title: "Qt Documentation"
  url: "https://doc.qt.io"
  accessed: "2026-04-24"
```

### thesis

```yaml
- id: ref.ivanov
  type: thesis
  authors: ["Ivanov I.I."]
  title: "Methods of Probabilistic Modelling"
  degree: "dis. ... kand. tekhn. nauk"   # optional, has a default value
  city: "Moscow"
  year: 2020
  pages: 150        # optional
```

---

## ID rules

Every `id` in the document must be unique across all sections, blocks, and bibliography entries. Duplicate IDs are reported as errors before compilation. File paths referenced in `figure`, `figure_group`, and `listing` blocks are checked for existence at load time.