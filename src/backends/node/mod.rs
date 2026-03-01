use crate::parser::ASTNode;

const RUNTIME: &str = include_str!("smorgasbord.js");

pub fn transpile_program(ast: &[ASTNode]) -> String {
    let mut output = String::from(RUNTIME);
    output.push('\n');
    
    let body: String = ast.iter()
        .map(|node| format!("{};", transpile_node(node)))
        .collect::<Vec<_>>()
        .join("\n");

    output.push_str(&body);
    output
}

pub fn transpile_node(node: &ASTNode) -> String {
    match node {
        ASTNode::Atom(s) => s.to_string(),
        ASTNode::List(list) => transpile_list(list),
    }
}

pub fn transpile_list(list: &[ASTNode]) -> String {
    let (head_node, args) = match list.split_first() {
        Some(res) => res,
        None => return String::new(),
    };

    let head = transpile_node(head_node);
    keyword_or_function(head, args)
}

fn transpile_arguments(args: &[ASTNode]) -> String {
    args.iter()
        .map(transpile_node)
        .collect::<Vec<String>>()
        .join(", ")
}

fn keyword_or_function(head: String, args: &[ASTNode]) -> String {
    match head.as_str() {
        "defconstant" => {
            if let (Some(name), Some(value)) = (args.get(0), args.get(1)) {
                format!("const {} = {}", transpile_node(name), transpile_node(value))
            } else {
                panic!("Error: Invalid defconstant arguments")
            }
        },
        "defun" => {
            if args.len() >= 3 {
                let name = transpile_node(&args[0]);
                let params = match &args[1] {
                    ASTNode::List(l) => transpile_arguments(l),
                    ASTNode::Atom(_) => panic!("Error: invalid defun arguments (args)"),
                };
                let body = transpile_node(&args[2]);
                format!("const {name} = ({params}) => {body}")
            } else {
                panic!("Error: Invalid defun arguments (ayo)")
            }
        },
        "if" => {
            if args.len() >= 2 {
                let condition = transpile_node(&args[0]);
                let if_do = transpile_node(&args[1]);

                println!("{condition} then {if_do} buddy");

                if let Some(if_false) = args.get(2) {
                    let else_do = transpile_node(if_false);

                    format!("({condition}) ? {if_do} : {else_do}")
                } else {
                    format!("({condition}) ? {if_do} : undefined")
                }
            } else {
                panic!("Error: Invalid if arguments")
            }
        },
        _ => format!("{}({})", head, transpile_arguments(args)),
    }
}
