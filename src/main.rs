use ide::{Analysis, DiagnosticsConfig};
use std::any::Any;

struct Node {
    func: Box<dyn Fn(Box<dyn Any>) -> Box<dyn Any>>,
    code: String,
    return_type: String,
    args: String,
}


fn compose(g: Node, f: Node) -> Node {
    let Node{func, code,  args, return_type} = f;
    let ffunc = func;
    let fcode = code;
    let freturn = return_type;
    let fargs = args;
    let Node{func, code, args, return_type, ..} = g;
    assert_eq!(args, freturn);
    Node{
        func: Box::new(move |x| func(ffunc(x))),
        code: fcode + code.as_str(), // temporary TODO: replace
        return_type,
        args: fargs,
    }
}

#[graph_proc_macros::to_node]
fn gen_int() -> (u32, u32) {
    (42,43)
}
#[graph_proc_macros::to_node]
fn format_int(x: u32, y: u32) -> String {
    x.to_string() + y.to_string().as_str()
}


pub fn check_code(code: String) {
    let (analysis, file_id) = Analysis::from_single_file(code);
    let config = DiagnosticsConfig::default();

    let diagnostics: Vec<_> = analysis
        .diagnostics(&config, ide::AssistResolveStrategy::All, file_id)
        .unwrap()
        .into_iter()
        .map(|d| d.message)
        .collect();

    println!("dig: {:#?}", diagnostics);
}

fn main() {
    let gen_int = gen_int();
    let format_int = format_int();
    let comp = compose(format_int, gen_int);

    println!("{:?}", (comp.func)(Box::new(())).downcast_ref::<String>());
    /*let function = TEST_FN;
    println!("{}", function);
    check_code(function.to_string());*/
}
