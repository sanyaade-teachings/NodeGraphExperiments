use ide::{Analysis, DiagnosticsConfig};


unsafe fn unsafe_compose<F: 'static, G: 'static, Fv, Fo, Gv, V>(g: G, f: F, ) -> Box<dyn Fn(Fv) -> V>
where
    F: Fn(Fv) -> Fo,
    G: Fn(Gv) -> V,
{
    Box::new(move |x| g(std::mem::transmute_copy::<Fo, Gv>(&f(x))))
}


#[graph_proc_macros::to_string]
fn gen_int(_: ()) -> u32 {
    42
}

#[graph_proc_macros::to_string]
fn format_int(x: u32) -> String {
    x.to_string()
}

pub fn check_code(code: String)  {
    let (analysis, file_id) = Analysis::from_single_file(code);
    let config = DiagnosticsConfig::default();

    let diagnostics: Vec<_> =analysis
        .diagnostics(&config, ide::AssistResolveStrategy::All, file_id)
        .unwrap()
        .into_iter()
        .map(|d| d.message)
        .collect();

        println!("dig: {:#?}", diagnostics);
}

fn main() {
    let comp = unsafe { unsafe_compose(format_int, gen_int) };
    println!("{}", comp(()));
    check_code(format_int_to_string().to_string());
}
