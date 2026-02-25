use crate::error::CliError;
use crate::output::CsvResultExt;
use crate::output::{
    Formattable, TableFormatter, generic_json_formatter, generic_json_list_formatter,
    generic_table_formatter, generic_table_list_formatter,
};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// A semantic search result from Linear
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticSearchResult {
    pub id: String,
    pub result_type: String,
    pub title: String,
    pub identifier: Option<String>,
    pub url: Option<String>,
}

impl TableFormatter for SemanticSearchResult {
    fn table_rows(&self) -> Vec<(Cow<'_, str>, Cow<'_, str>)> {
        let mut rows = vec![
            (
                Cow::Borrowed("Type"),
                Cow::Borrowed(self.result_type.as_str()),
            ),
            (Cow::Borrowed("Title"), Cow::Borrowed(self.title.as_str())),
        ];
        if let Some(id) = &self.identifier {
            rows.push((Cow::Borrowed("Identifier"), Cow::Borrowed(id.as_str())));
        }
        if let Some(url) = &self.url {
            rows.push((Cow::Borrowed("URL"), Cow::Borrowed(url.as_str())));
        }
        rows
    }
}

impl Formattable for SemanticSearchResult {
    fn to_json(&self) -> Result<String, CliError> {
        generic_json_formatter(self)
    }

    fn to_csv(&self) -> Result<String, CliError> {
        let mut wtr = csv::Writer::from_writer(vec![]);

        wtr.write_record(["id", "result_type", "title", "identifier", "url"])
            .csv_err("Failed to write CSV header")?;

        wtr.write_record([
            &self.id,
            &self.result_type,
            &self.title,
            self.identifier.as_deref().unwrap_or(""),
            self.url.as_deref().unwrap_or(""),
        ])
        .csv_err("Failed to write CSV row")?;

        let bytes = wtr
            .into_inner()
            .map_err(|e| CliError::General(format!("CSV flush error: {e}")))?;
        String::from_utf8(bytes).map_err(|e| CliError::General(format!("UTF-8 error: {e}")))
    }

    fn to_markdown(&self) -> Result<String, CliError> {
        use std::fmt::Write;
        let mut output = String::new();

        let display = self.identifier.as_deref().unwrap_or(&self.title);
        writeln!(output, "### {display}")
            .map_err(|e| CliError::General(format!("Failed to write markdown: {e}")))?;
        writeln!(
            output,
            "**Type:** {} | **Title:** {}",
            self.result_type, self.title
        )
        .map_err(|e| CliError::General(format!("Failed to write markdown: {e}")))?;
        if let Some(url) = &self.url {
            writeln!(output, "**URL:** {url}")
                .map_err(|e| CliError::General(format!("Failed to write markdown: {e}")))?;
        }

        Ok(output)
    }

    fn to_table(&self) -> Result<String, CliError> {
        generic_table_formatter(self)
    }
}

/// Wrapper for a list of semantic search results
pub struct SemanticSearchResultList(pub Vec<SemanticSearchResult>);

impl Formattable for SemanticSearchResultList {
    fn to_json(&self) -> Result<String, CliError> {
        generic_json_list_formatter(&self.0)
    }

    fn to_csv(&self) -> Result<String, CliError> {
        let mut wtr = csv::Writer::from_writer(vec![]);

        wtr.write_record(["result_type", "title", "identifier", "url"])
            .csv_err("Failed to write CSV header")?;

        for r in &self.0 {
            wtr.write_record([
                &r.result_type,
                &r.title,
                r.identifier.as_deref().unwrap_or(""),
                r.url.as_deref().unwrap_or(""),
            ])
            .csv_err("Failed to write CSV row")?;
        }

        let bytes = wtr
            .into_inner()
            .map_err(|e| CliError::General(format!("CSV flush error: {e}")))?;
        String::from_utf8(bytes).map_err(|e| CliError::General(format!("UTF-8 error: {e}")))
    }

    fn to_markdown(&self) -> Result<String, CliError> {
        use std::fmt::Write;
        let mut output = String::new();

        writeln!(output, "## Search Results ({})\n", self.0.len())
            .map_err(|e| CliError::General(format!("Failed to write markdown: {e}")))?;

        for r in &self.0 {
            let display = r.identifier.as_deref().unwrap_or(&r.title);
            writeln!(
                output,
                "- **[{type}]** {display}: {title}",
                r#type = r.result_type,
                title = r.title,
            )
            .map_err(|e| CliError::General(format!("Failed to write markdown: {e}")))?;
        }

        Ok(output)
    }

    fn to_table(&self) -> Result<String, CliError> {
        generic_table_list_formatter(&self.0, &["Type", "Identifier", "Title"], |r| {
            vec![
                r.result_type.clone(),
                r.identifier.clone().unwrap_or_else(|| "—".to_string()),
                r.title.clone(),
            ]
        })
    }
}
