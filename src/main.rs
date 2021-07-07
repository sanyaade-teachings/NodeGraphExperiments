use ide::{Analysis, CompletionConfig, DiagnosticsConfig, FileId, FilePosition, Indel, TextSize};
use ide_db::helpers::{
    insert_use::{InsertUseConfig, PrefixKind},
    SnippetCap,
};

struct State {
    analysis: Analysis,
    file_id: FileId,
}

impl State {
    fn new() -> State {
         let (analysis, file_id) = Analysis::from_single_file("".to_owned());
        Self { analysis, file_id }
    }
    pub fn update(&mut self, code: String)  {
        let (analysis, file_id) = Analysis::from_single_file(code);
        self.analysis = analysis;
        self.file_id = file_id;

        let line_index = self.analysis.file_line_index(self.file_id).unwrap();

        let highlights: Vec<_> = self
            .analysis
            .highlight(file_id)
            .unwrap()
            .into_iter()
            /*.map(|hl| Highlight {
                tag: Some(hl.highlight.tag.to_string()),
                range: to_proto::text_range(hl.range, &line_index),
            })*/
            .collect();

        let config = DiagnosticsConfig::default();

        let diagnostics: Vec<_> = self
            .analysis
            .diagnostics(&config, ide::AssistResolveStrategy::All, file_id)
            .unwrap()
            .into_iter()
            /*.map(|d| {
                let Range { startLineNumber, startColumn, endLineNumber, endColumn } =
                    to_proto::text_range(d.range, &line_index);
                Diagnostic {
                    message: d.message,
                    severity: to_proto::severity(d.severity),
                    startLineNumber,
                    startColumn,
                    endLineNumber,
                    endColumn,
                }
            })*/
            .collect();

            println!("dig: {:#?}", diagnostics);
            println!("hig: {:?}", highlights);
    }
}

fn main() {
    let mut state = State::new();

    state.update("fn a() {}".to_string());

    println!("Hello, world!");
}
