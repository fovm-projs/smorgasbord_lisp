#[derive(Debug, Clone)]
pub enum ASTNode {
    List(Vec<ASTNode>),
    Atom(String),
}

pub fn parse(input: &str) -> Vec<ASTNode> {
    let mut nodes: Vec<ASTNode> = Vec::new();
    let mut stack: Vec<usize> = Vec::new();
    let mut buffer = String::new();
    let mut in_quotes = false;
    let mut chars = input.chars().peekable();

    let flush_buffer = |nodes: &mut Vec<ASTNode>, buffer: &mut String| {
        if !buffer.is_empty() {
            nodes.push(ASTNode::Atom(std::mem::take(buffer)));
        }
    };

    while let Some(ch) = chars.next() {
        if in_quotes {
            if ch == '"' {
                in_quotes = false;
                buffer.push('"');

                flush_buffer(&mut nodes, &mut buffer);
            } else {
                buffer.push(ch);
            }
            continue;
        }

        match ch {
            '"' => {
                flush_buffer(&mut nodes, &mut buffer);
                buffer.push('"');
                in_quotes = true;
            }
            ';' => {
                flush_buffer(&mut nodes, &mut buffer);
                while let Some(&next_ch) = chars.peek() {
                    if next_ch == '\n' { break; }
                    chars.next();
                }
            }
            '(' => {
                flush_buffer(&mut nodes, &mut buffer);
                stack.push(nodes.len());
            }
            ')' => {
                flush_buffer(&mut nodes, &mut buffer);
                let start = stack.pop().expect("лишняя ')'");
                let list: Vec<ASTNode> = nodes.drain(start..).collect();
                nodes.push(ASTNode::List(list));
            }
            c if c.is_whitespace() => {
                flush_buffer(&mut nodes, &mut buffer);
            }
            _ => buffer.push(ch),
        }
    }

    flush_buffer(&mut nodes, &mut buffer);
    if in_quotes { panic!("незакрытая кавычка"); }
    if !stack.is_empty() { panic!("несбалансированные скобки"); }

    nodes
}
