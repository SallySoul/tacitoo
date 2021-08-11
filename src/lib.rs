use std::fmt::Write;

// pub mod interval;

// We have
// * Tree of Nodes describing eqt
// * Interface of implicit expressions required for rendering
//   * Expression interface
// * Variables in expressions
//   * Variables really only need an id
//   * Sometimes we want to debug them though, good to have name map too?
// * Intervals would be good to collect in item
//   * Subset?

pub trait Expression: Sized {
    fn evaluate_point(&self /* VariablesBindings */);

    // Derivative, do we want jacobian? or should we give dx / dy style query?
    fn derivative_point(&self /* Derivative Point */) /* -> Matrix / Derivative type */;

    fn evaluate_interval(&self /* Intervals */);
}

pub type NodeId = usize;

#[derive(PartialEq, Debug)]
pub enum Node {
    Add(NodeId, NodeId),
    Sub(NodeId, NodeId),
    Mul(NodeId, NodeId),
    Div(NodeId, NodeId),
    Exp(NodeId, NodeId),
    Variable(usize),
    Constant(f32),
}

pub struct NodeExpression {
    nodes: Vec<Node>,
    root: NodeId,
}

impl NodeExpression {
    pub fn new() -> NodeExpression {
        NodeExpression {
            nodes: Vec::new(),
            root: 0,
        }
    }

    pub fn add(&mut self, n: Node) -> NodeId {
        let result = self.nodes.len();
        self.nodes.push(n);
        result
    }

    pub fn node(&self, n: NodeId) -> &Node {
        &self.nodes[n]
    }

    pub fn format(&self, n: NodeId) -> String {
        let mut result = String::new();

        enum FormatToken {
            Str(&'static str),
            Expr(NodeId),
        }

        let mut stack = Vec::new();
        stack.push(FormatToken::Expr(n));
        while !stack.is_empty() {
            match stack.pop().unwrap() {
                FormatToken::Str(s) => {
                    result += s;
                },
                FormatToken::Expr(n) => {
                    match self.node(n) {
                        Node::Add(a, b) => {
                            stack.push(FormatToken::Str(")"));
                            stack.push(FormatToken::Expr(b.clone()));
                            stack.push(FormatToken::Str(" + "));
                            stack.push(FormatToken::Expr(a.clone()));
                            stack.push(FormatToken::Str("("));
                        }
                        Node::Sub(a, b) => {
                            stack.push(FormatToken::Str(")"));
                            stack.push(FormatToken::Expr(b.clone()));
                            stack.push(FormatToken::Str(" - "));
                            stack.push(FormatToken::Expr(a.clone()));
                            stack.push(FormatToken::Str("("));
                        }
                        Node::Mul(a, b) => {
                            stack.push(FormatToken::Str(")"));
                            stack.push(FormatToken::Expr(b.clone()));
                            stack.push(FormatToken::Str(" * "));
                            stack.push(FormatToken::Expr(a.clone()));
                            stack.push(FormatToken::Str("("));
                        }

                        Node::Div(a, b) => {
                            stack.push(FormatToken::Str(")"));
                            stack.push(FormatToken::Expr(b.clone()));
                            stack.push(FormatToken::Str(" / "));
                            stack.push(FormatToken::Expr(a.clone()));
                            stack.push(FormatToken::Str("("));
                        }

                        Node::Exp(a, b) => {
                            stack.push(FormatToken::Str(")"));
                            stack.push(FormatToken::Expr(b.clone()));
                            stack.push(FormatToken::Str(" ^ "));
                            stack.push(FormatToken::Expr(a.clone()));
                            stack.push(FormatToken::Str("("));
                        }
                        Node::Variable(id) => {
                            write!(&mut result, "Var({})", id).unwrap();
                        }
                        Node::Constant(c) => {
                            write!(&mut result, "{}", c).unwrap();
                        }
                    }
                }
            }
        }

        result
    }

    pub fn evaluate_point(&self, n: NodeId, bindings: &[f32]) -> f32 {
        #[derive(Debug)]
        enum EvalToken {
            Val(f32),
            Add,
            Sub,
            Mul,
            Div,
            Exp,
            Expr(NodeId),
        }

        let mut stack: Vec<EvalToken> = Vec::new();
        stack.push(EvalToken::Expr(n));

        loop {
            match stack.len() {
                0 => {
                    panic!("Empty Stack, shouldn't happen");
                }
                1 => {
                    match stack[0] {
                        EvalToken::Val(x) => {
                            return x;
                        }
                        EvalToken::Expr(_) => (),
                        token => {
                            panic!("Last element in stack wasn't a value or expr, {:?}", token)
                        }
                    }
                }
                _ => (),
            }

            match stack.pop().unwrap() {
                EvalToken::Val(_) => {
                    panic!("I don't think this should happen");
                },
                EvalToken::Add => {
                    assert!(stack.len() >= 2);
                    match (stack.pop().unwrap(), stack.pop().unwrap()) {
                        (EvalToken::Val(a), EvalToken::Val(b)) => {
                            stack.push(EvalToken::Val(a + b));
                        },
                        _ => {
                            panic!("I don't think this should happen");
                        }
                    }
                },
                EvalToken::Sub => {
                    assert!(stack.len() >= 2);
                    match (stack.pop().unwrap(), stack.pop().unwrap()) {
                        (EvalToken::Val(a), EvalToken::Val(b)) => {
                            stack.push(EvalToken::Val(a - b));
                        },
                        _ => {
                            panic!("I don't think this should happen");
                        }
                    }
                },
                EvalToken::Mul => {
                    assert!(stack.len() >= 2);
                    match (stack.pop().unwrap(), stack.pop().unwrap()) {
                        (EvalToken::Val(a), EvalToken::Val(b)) => {
                            stack.push(EvalToken::Val(a * b));
                        },
                        _ => {
                            panic!("I don't think this should happen");
                        }
                    }
                },
                EvalToken::Div => {
                    assert!(stack.len() >= 2);
                    match (stack.pop().unwrap(), stack.pop().unwrap()) {
                        (EvalToken::Val(a), EvalToken::Val(b)) => {
                            stack.push(EvalToken::Val(a / b));
                        },
                        _ => {
                            panic!("I don't think this should happen");
                        }
                    }
                }
                EvalToken::Exp => {
                    assert!(stack.len() >= 2);
                    match (stack.pop().unwrap(), stack.pop().unwrap()) {
                        (EvalToken::Val(a), EvalToken::Val(b)) => {
                            stack.push(EvalToken::Val(a.powf(b)));
                        },
                        _ => {
                            panic!("I don't think this should happen");
                        }
                    }
                }
                EvalToken::Expr(n) => {
                    match self.node(n) {
                        Node::Add(a, b) => {
                            stack.push(EvalToken::Add);
                            stack.push(EvalToken::Expr(b.clone()));
                            stack.push(EvalToken::Expr(a.clone()));
                        },
                        Node::Sub(a, b) => {
                            stack.push(EvalToken::Expr(b.clone()));
                            stack.push(EvalToken::Expr(a.clone()));
                            stack.push(EvalToken::Sub);
                        },
                        Node::Mul(a, b) => {
                            stack.push(EvalToken::Expr(b.clone()));
                            stack.push(EvalToken::Expr(a.clone()));
                            stack.push(EvalToken::Mul);
                        },
                        Node::Div(a, b) => {
                            stack.push(EvalToken::Expr(b.clone()));
                            stack.push(EvalToken::Expr(a.clone()));
                            stack.push(EvalToken::Div);
                        },
                        Node::Exp(a, b) => {
                            stack.push(EvalToken::Expr(b.clone()));
                            stack.push(EvalToken::Expr(a.clone()));
                            stack.push(EvalToken::Exp);
                        },
                        Node::Variable(id) => {
                            stack.push(EvalToken::Val(bindings[id.clone()]));
                        }
                        Node::Constant(c) => {
                            stack.push(EvalToken::Val(c.clone()));
                        }
                    }
                }
            }
        }
*/
        0.0
    }


/*
    pub fn partial_derivative(&mut self, n: NodeId, dx: usize, dy: usize) -> NodeId {
        enum DerivToken {
            Expr(NodeId),
        }
       let mut stack = Vec::new();
       stack.push(DerivToken::Expr(n));
       while !stack.is_empty() {
           match stack.pop() {
               Node::Add(a, b) => {
                            stack.push(DeriveToken::Expr("b.clone())");
                            stack.push(FormatToken::Expr(b.clone()));
                            stack.push(FormatToken::Str(" + "));
                            stack.push(FormatToken::Expr(a.clone()));
                            stack.push(FormatToken::Str("("));
                        }
                        Node::Sub(a, b) => {
                            stack.push(FormatToken::Str(")"));
                            stack.push(FormatToken::Expr(b.clone()));
                            stack.push(FormatToken::Str(" - "));
                            stack.push(FormatToken::Expr(a.clone()));
                            stack.push(FormatToken::Str("("));
                        }
                        Node::Mul(a, b) => {
                            stack.push(FormatToken::Str(")"));
                            stack.push(FormatToken::Expr(b.clone()));
                            stack.push(FormatToken::Str(" * "));
                            stack.push(FormatToken::Expr(a.clone()));
                            stack.push(FormatToken::Str("("));
                        }

                        Node::Div(a, b) => {
                            stack.push(FormatToken::Str(")"));
                            stack.push(FormatToken::Expr(b.clone()));
                            stack.push(FormatToken::Str(" / "));
                            stack.push(FormatToken::Expr(a.clone()));
                            stack.push(FormatToken::Str("("));
                        }

                        Node::Exp(a, b) => {
                            stack.push(FormatToken::Str(")"));
                            stack.push(FormatToken::Expr(b.clone()));
                            stack.push(FormatToken::Str(" ^ "));
                            stack.push(FormatToken::Expr(a.clone()));
                            stack.push(FormatToken::Str("("));
                        }
                        Node::Variable(id) => {
                            write!(&mut result, "Var({})", id).unwrap();
                        }
                        Node::Constant(c) => {
                            write!(&mut result, "{}", c).unwrap();
                        }
           }
       }


       0
    }
*/
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format() {
        {
            let mut nodes = NodeExpression::new();
            let n0 = nodes.add(Node::Constant(5.0));
            let n1 = nodes.add(Node::Variable(0));
            let n2 = nodes.add(Node::Variable(1));
            let n3 = nodes.add(Node::Add(n0, n1));
            let n4 = nodes.add(Node::Mul(n3, n2));
            assert_eq!(nodes.format(n4), "((5 + Var(0)) * Var(1))");
        }
    }

    fn evaluate() {
        {
            let mut nodes = NodeExpression::new();
            let n0 = nodes.add(Node::Constant(5.0));
            let n1 = nodes.add(Node::Variable(0));
            let n2 = nodes.add(Node::Variable(1));
            let n3 = nodes.add(Node::Add(n0, n1));
            let n4 = nodes.add(Node::Mul(n3, n2));
            assert_eq!(nodes.format(n4), "((5 + Var(0)) * Var(1))");
        }
    }
}
