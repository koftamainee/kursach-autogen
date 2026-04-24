use serde::Deserialize;
use std::path::PathBuf;

// ── Top-level ────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct Document {
    pub meta: Meta,
    pub style: Style,
    pub document: Vec<Section>,
}

// ── Meta ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct Meta {
    pub university: String,
    pub department: String,
    pub subject: String,
    pub title: String,
    pub author: Person,
    pub supervisor: Supervisor,
    pub year: u32,
    pub city: String,
    #[serde(default)]
    pub logo: Option<PathBuf>,
    #[serde(default)]
    pub abstract_: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Person {
    pub name: String,
    pub group: String,
}

#[derive(Debug, Deserialize)]
pub struct Supervisor {
    pub name: String,
    pub title: String,
}

// ── Style ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct Style {
    pub page: PageStyle,
    pub fonts: FontSet,
    pub figures: CaptionStyle,
    pub tables: CaptionStyle,
    pub listings: CaptionStyle,
    pub bibliography: BibStyle,
}

#[derive(Debug, Deserialize)]
pub struct PageStyle {
    pub margins: Margins,
    pub numbering: PageNumbering,
}

#[derive(Debug, Deserialize)]
pub struct Margins {
    pub left: String,
    pub right: String,
    pub top: String,
    pub bottom: String,
}

#[derive(Debug, Deserialize)]
pub struct PageNumbering {
    pub start_from: u32,
    pub show_on_title: bool,
}

#[derive(Debug, Deserialize)]
pub struct FontSet {
    pub main: FontSpec,
    pub heading: FontSpec,
    pub caption: FontSpec,
    pub listing_body: FontSpec,
}

#[derive(Debug, Deserialize)]
pub struct FontSpec {
    pub family: String,
    pub size: String,
    pub line_spacing: f32,
    #[serde(default)]
    pub align: Option<String>,
    #[serde(default)]
    pub bold: Option<bool>,
    #[serde(default)]
    pub italic: Option<bool>,
    #[serde(default)]
    pub indent: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct CaptionStyle {
    #[serde(default)]
    pub align: Option<String>,
    pub caption_position: String,
    pub caption_format: String,
}

#[derive(Debug, Deserialize)]
pub struct BibStyle {
    pub standard: String,
    pub label_format: String,
}

// ── Sections ──────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct Section {
    pub id: String,
    pub title: String,
    #[serde(default)]
    pub numbered: bool,
    #[serde(flatten)]
    pub kind: SectionKind,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum SectionKind {
    Bibliography {
        entries: Vec<BibEntry>,
    },
    Regular {
        #[serde(default)]
        body: Vec<Block>,
        #[serde(default)]
        subsections: Vec<Section>,
    },
}

// ── Blocks ────────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Block {
    Paragraph {
        text: String,
    },
    Formula {
        #[serde(default)]
        id: Option<String>,
        content: String,
    },
    Figure {
        id: String,
        path: PathBuf,
        caption: String,
        #[serde(default = "default_width")]
        width: f32,
    },
    FigureGroup {
        id: String,
        caption: String,
        #[serde(default)]
        layout: FigureGroupLayout,
        figures: Vec<SubFigure>,
    },
    Listing {
        id: String,
        #[serde(flatten)]
        source: ListingSource,
        language: String,
        caption: String,
        #[serde(default)]
        range: Option<(usize, usize)>,
    },
    Table {
        id: String,
        caption: String,
        columns: Vec<String>,
        rows: Vec<Vec<String>>,
    },
    List {
        #[serde(default)]
        ordered: bool,
        items: Vec<ListItem>,
    },
    Algorithm {
        id: String,
        caption: String,
        #[serde(default)]
        numbered: bool,
        steps: Vec<AlgoStep>,
    },
    Note {
        text: String,
        #[serde(default)]
        title: Option<String>,
    },
    Warning {
        text: String,
        #[serde(default)]
        title: Option<String>,
    },
    PageBreak,
    RawLatex {
        content: String,
    },
}

#[derive(Debug, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum AlgoStep {
    Statement { text: String },
    Require { text: String },
    Ensure { text: String },
    Return { text: String },
    If {
        cond: String,
        then: Vec<AlgoStep>,
        #[serde(default)]
        else_: Vec<AlgoStep>,
    },
    For {
        var: String,
        then: Vec<AlgoStep>,
    },
    While {
        cond: String,
        then: Vec<AlgoStep>,
    },
    Comment { text: String },
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum FigureGroupLayout {
    #[default]
    Horizontal,
    Vertical,
}

#[derive(Debug, Deserialize)]
pub struct SubFigure {
    pub path: PathBuf,
    pub subcaption: String,
    #[serde(default = "default_width")]
    pub width: f32,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ListItem {
    Text(String),
    Nested(Box<Block>),
}

fn default_width() -> f32 {
    0.8
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ListingSource {
    File { path: PathBuf },
    Inline { content: String },
}

// ── Bibliography ──────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct BibEntry {
    pub id: String,
    #[serde(flatten)]
    pub kind: BibKind,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum BibKind {
    Book {
        authors: Vec<String>,
        title: String,
        publisher: String,
        year: u32,
        city: String,
        #[serde(default)]
        pages: Option<u32>,
    },
    Article {
        authors: Vec<String>,
        title: String,
        journal: String,
        year: u32,
        #[serde(default)]
        volume: Option<String>,
        #[serde(default)]
        pages: Option<String>,
    },
    Online {
        #[serde(default)]
        authors: Vec<String>,
        title: String,
        url: String,
        accessed: String,
    },
    Thesis {
        authors: Vec<String>,
        title: String,
        #[serde(default)]
        degree: Option<String>,
        city: String,
        year: u32,
        #[serde(default)]
        pages: Option<u32>,
    },
}