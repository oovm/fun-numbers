use std::{
    fmt::{Debug, Formatter, Write},
    ops::{Add, Div, Mul, Sub},
};

use ahash::RandomState;
use dashmap::DashMap;
use dashu::{integer::IBig, rational::RBig};

use crate::{ExpressionAction, ExpressionNode, StopReason};

pub type NodeID = usize;

#[derive(Default, Debug)]
pub struct ExpressionPool {
    cache: DashMap<NodeID, EvaluatedState, RandomState>,
}

#[derive(Clone)]
pub struct EvaluatedState {
    is_initial: bool,
    is_evaluated: bool,
    expression: ExpressionNode,
    result: RBig,
    failure: Option<StopReason>,
}

impl Debug for EvaluatedState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.failure {
            Some(s) => f.debug_struct("EvaluateFailure").field("reason", &s).finish(),
            None if self.is_evaluated => {
                f.debug_struct("EvaluateSuccess").field("expression", &self.expression).field("result", &self.result).finish()
            }
            None => f.debug_struct("EvaluatePending").field("expression", &self.expression).finish(),
        }
    }
}

impl EvaluatedState {
    pub fn initial(number: usize) -> EvaluatedState {
        Self {
            is_initial: true,
            is_evaluated: true,
            expression: ExpressionNode::Atomic { number },
            result: RBig::from(number),
            failure: None,
        }
    }
    pub fn get_node_id(&self) -> NodeID {
        self.expression.get_id()
    }
    pub fn get_priority(&self) -> usize {
        self.expression.get_priority()
    }
}

impl ExpressionPool {
    pub fn evaluate(&mut self, node: &NodeID) -> Result<RBig, StopReason> {
        let mut node = self.find(node)?;
        if node.is_evaluated {
            return Ok(node.result.clone());
        }
        match self.try_evaluate(&node) {
            Ok(o) => Ok(self.update_success(node, o)),
            Err(e) => Err(self.update_failure(node, e)),
        }
    }
    fn try_evaluate(&mut self, node: &EvaluatedState) -> Result<RBig, StopReason> {
        let out = match &node.expression {
            ExpressionNode::Atomic { .. } => unreachable!("All atomic nodes should be evaluated"),
            ExpressionNode::Binary { lhs, rhs, action } => match action {
                ExpressionAction::Concat => {
                    if !node.expression.is_atomic_concat(self) {
                        Err(StopReason::NonAtomicConcat)?;
                    }
                    let lhs = self.evaluate(lhs)?;
                    let rhs = self.evaluate(rhs)?;
                    lhs.mul(RBig::from(10)).add(rhs)
                }
                ExpressionAction::Plus => {
                    let lhs = self.evaluate(lhs)?;
                    let rhs = self.evaluate(rhs)?;
                    lhs.add(rhs)
                }
                ExpressionAction::Minus => {
                    let lhs = self.evaluate(lhs)?;
                    let rhs = self.evaluate(rhs)?;
                    lhs.sub(rhs)
                }
                ExpressionAction::Times => {
                    let lhs = self.evaluate(lhs)?;
                    let rhs = self.evaluate(rhs)?;
                    lhs.mul(rhs)
                }
                ExpressionAction::Divide => {
                    let lhs = self.evaluate(lhs)?;
                    let rhs = self.evaluate(rhs)?;
                    if rhs.is_zero() {
                        return Err(StopReason::DividedByZero);
                    }
                    lhs.div(rhs)
                }
            },
        };
        Ok(out)
    }
    pub fn insert_atomic(&mut self, number: usize) -> NodeID {
        let out = EvaluatedState {
            is_initial: true,
            is_evaluated: true,
            expression: ExpressionNode::Atomic { number },
            result: RBig::from(number),
            failure: None,
        };
        self.do_insert(out)
    }
    pub fn insert_binary(&mut self, action: ExpressionAction, lhs: NodeID, rhs: NodeID) -> NodeID {
        let out = EvaluatedState {
            is_initial: false,
            is_evaluated: false,
            expression: ExpressionNode::Binary { lhs, rhs, action },
            result: RBig::default(),
            failure: None,
        };
        self.do_insert(out)
    }
    fn do_insert(&mut self, node: EvaluatedState) -> NodeID {
        let id = node.get_node_id();
        match self.cache.get(&id) {
            Some(s) if s.is_evaluated => return id,
            _ => {}
        }
        self.cache.insert(id, node);
        id
    }

    pub fn update_success(&mut self, mut state: EvaluatedState, result: RBig) -> RBig {
        state.is_evaluated = true;
        state.result = result.clone();
        self.cache.insert(state.get_node_id(), state);
        result
    }
    pub fn update_failure(&mut self, mut state: EvaluatedState, reason: StopReason) -> StopReason {
        state.is_evaluated = true;
        state.failure = Some(reason.clone());
        self.cache.insert(state.get_node_id(), state);
        reason
    }
    pub fn find(&self, node: &NodeID) -> Result<EvaluatedState, StopReason> {
        match self.cache.get(node) {
            Some(s) => match &s.failure {
                Some(s) => Err(s.clone()),
                None => Ok(s.clone()),
            },
            None => Err(StopReason::NotFound),
        }
    }
    pub fn rewrite<W: Write>(&self, node: &NodeID, w: &mut W) -> Result<(), StopReason> {
        let state = self.cache.get(node).ok_or(StopReason::NotFound)?;
        match state.expression {
            ExpressionNode::Atomic { number } => {
                write!(w, "{}", number)?;
            }
            ExpressionNode::Binary { lhs, rhs, action } => match action {
                ExpressionAction::Concat => {
                    self.rewrite(&lhs, w)?;
                    self.rewrite(&rhs, w)?;
                }
                ExpressionAction::Plus => {
                    self.rewrite(&lhs, w)?;
                    write!(w, " + ")?;
                    self.rewrite(&rhs, w)?;
                }
                ExpressionAction::Minus => {
                    self.rewrite(&lhs, w)?;
                    write!(w, " - ")?;
                    self.rewrite(&rhs, w)?;
                }
                ExpressionAction::Times => {
                    if self.should_add_brackets(node, &lhs) {
                        write!(w, "(")?;
                        self.rewrite(&lhs, w)?;
                        write!(w, ")")?;
                    }
                    else {
                        self.rewrite(&lhs, w)?;
                    }
                    write!(w, " * ")?;
                    if self.should_add_brackets(node, &rhs) {
                        write!(w, "(")?;
                        self.rewrite(&rhs, w)?;
                        write!(w, ")")?;
                    }
                    else {
                        self.rewrite(&rhs, w)?;
                    }
                }
                ExpressionAction::Divide => {
                    if self.should_add_brackets(node, &lhs) {
                        write!(w, "(")?;
                        self.rewrite(&lhs, w)?;
                        write!(w, ")")?;
                    }
                    else {
                        self.rewrite(&lhs, w)?;
                    }
                    write!(w, " / ")?;
                    if self.should_add_brackets(node, &rhs) {
                        write!(w, "(")?;
                        self.rewrite(&rhs, w)?;
                        write!(w, ")")?;
                    }
                    else {
                        self.rewrite(&rhs, w)?;
                    }
                }
            },
        }
        Ok(())
    }
    fn should_add_brackets(&self, node: &NodeID, parent: &NodeID) -> bool {
        let node = match self.cache.get(node) {
            Some(s) => s.get_priority(),
            None => return true,
        };
        let parent = match self.cache.get(parent) {
            Some(s) => s.get_priority(),
            None => return true,
        };
        node < parent
    }
}

impl ExpressionNode {
    pub fn is_atomic_concat(&self, pool: &ExpressionPool) -> bool {
        match self {
            ExpressionNode::Atomic { .. } => true,
            ExpressionNode::Binary { lhs, rhs, action } => match (pool.cache.get(lhs), pool.cache.get(rhs)) {
                (Some(lhs), Some(rhs)) => lhs.expression.is_atomic_concat(pool) && rhs.expression.is_atomic_concat(pool),
                _ => false,
            },
            _ => false,
        }
    }
}

pub fn evaluate(id: NodeID, pool: &mut ExpressionPool) -> Result<IBig, StopReason> {
    let result = pool.evaluate(&id)?.into_parts();
    if !result.1.is_one() {
        Err(StopReason::NotInteger)?
    }
    Ok(result.0)
}

#[test]
fn debug() {
    let mut pool = ExpressionPool::default();
    let lhs = pool.insert_atomic(1);
    let rhs = pool.insert_atomic(2);
    let id = pool.insert_binary(ExpressionNode::Add { lhs, rhs });
    let mut expression = String::new();
    pool.rewrite(&id, &mut expression).unwrap();
    println!("{:#?}", evaluate(id, &mut pool));
    println!("{:#?}", expression);
}

#[test]
fn debug2() {
    println!("{:#?}", RBig::from(3).div(RBig::from(4)).div(RBig::from(5)));
}
