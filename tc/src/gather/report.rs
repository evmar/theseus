pub struct IATEntry {
    pub addr: u32,
    pub func: String,
}

#[derive(Default)]
pub struct Report {
    pub name: String,
    pub iat: Vec<IATEntry>,
}

fn table<'a>(header: &[&str], rows: impl Iterator<Item = Vec<String>>) -> String {
    let mut out = String::new();
    out.push_str("<table>");
    out.push_str("<tr>");
    for cell in header {
        out.push_str(&format!("<th>{cell}</th>"));
    }
    out.push_str("</tr>");
    for row in rows {
        out.push_str("<tr>");
        for cell in row {
            out.push_str(&format!("<td>{cell}</td>"));
        }
        out.push_str("</tr>");
    }
    out.push_str("</table>");
    out
}

impl Report {
    pub fn to_html(mut self) -> String {
        self.iat.sort_by_key(|e| e.addr);

        let style = [
            "body { font-family: system-ui, sans-serif; font-size: 14px; font-variant-numeric: tabular-nums;",
            " margin: 3em 4em; }",
            "h1, h2 { font-weight: normal; }",
            "table {
                text-align: left;
            }",
        ]
        .join("\n");
        let title = format!("<h1>Theseus analysis of {name}</h1>", name = self.name);
        let html_iat = table(
            &["addr", "func"],
            self.iat
                .into_iter()
                .map(|IATEntry { addr, func }| vec![format!("{:x}", addr), func]),
        );
        [
            "<!doctype html>",
            &format!("<style>{style}</style>"),
            &title,
            "<h2>IAT</h2>",
            &html_iat,
        ]
        .join("\n")
    }
}
