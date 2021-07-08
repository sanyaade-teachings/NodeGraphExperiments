use std::any::Any;

macro_rules! eval {
    ($n:expr, $t:expr) => {{
        ($n.func)(Box::new($t)).downcast().unwrap()
    }};
    ($n:expr, $t:expr, $ty:ty) => {{
        ($n.func)(Box::new($t)).downcast::<$ty>().unwrap()
    }}
}

struct Node {
    func: Box<dyn Fn(Box<dyn Any>) -> Box<dyn Any>>,
    code: String,
    return_type: String,
    args: String,
}

impl std::ops::Mul<Self> for Node {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        compose(self, other)
    }
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


#[cfg(rust_analyzer)]
pub fn check_code(code: String) {
    use ide::{Analysis, DiagnosticsConfig};
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
    println!("{:?}", eval!(format_int() * gen_int(), (), String));
    let function = format_int().code;
    println!("{}", function);
    #[cfg(rust_analyzer)]
    check_code(function.to_string());
}
