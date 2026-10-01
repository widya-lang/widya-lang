//! Dynamic Computational Graph & Automatic Differentiation Engine (autograd.rs)
//! Mendukung Reverse-Mode Automatic Differentiation (Backpropagation), penelusuran gradien otomatis, dan optimasi loss function.

use std::collections::HashMap;

/// Operasi komputasi yang direkam dalam dynamic graph
#[derive(Debug, Clone, PartialEq)]
pub enum GraphOp {
    Leaf,
    Add(usize, usize),      // lhs_id, rhs_id
    Sub(usize, usize),
    Mul(usize, usize),
    Scale(usize, f32),      // tensor_id, scalar
    MatMul(usize, usize),   // lhs_id, rhs_id
    ReLU(usize),
    Sigmoid(usize),
    MSELoss(usize, usize),  // pred_id, target_id
}

/// Node Tensor di dalam Dynamic Computational Graph
#[derive(Debug, Clone)]
pub struct AutoGradNode {
    pub id: usize,
    pub data: Vec<f32>,
    pub grad: Vec<f32>,
    pub shape: Vec<usize>,
    pub op: GraphOp,
    pub requires_grad: bool,
}

/// Dynamic Computational Graph Manager
pub struct AutoGradEngine {
    nodes: Vec<AutoGradNode>,
}

impl AutoGradEngine {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    /// Buat Leaf Variable (Input atau Parameter Model Bobot/Bias)
    pub fn variable(&mut self, shape: Vec<usize>, data: Vec<f32>, requires_grad: bool) -> usize {
        let id = self.nodes.len();
        let len: usize = shape.iter().product();
        assert_eq!(data.len(), len);

        self.nodes.push(AutoGradNode {
            id,
            data,
            grad: vec![0.0; len],
            shape,
            op: GraphOp::Leaf,
            requires_grad,
        });

        id
    }

    /// Operasi Penjumlahan (Add)
    pub fn add(&mut self, a_id: usize, b_id: usize) -> usize {
        let len = self.nodes[a_id].data.len();
        let mut out_data = vec![0.0; len];
        for i in 0..len {
            out_data[i] = self.nodes[a_id].data[i] + self.nodes[b_id].data[i];
        }

        let id = self.nodes.len();
        let shape = self.nodes[a_id].shape.clone();
        let req_grad = self.nodes[a_id].requires_grad || self.nodes[b_id].requires_grad;

        self.nodes.push(AutoGradNode {
            id,
            data: out_data,
            grad: vec![0.0; len],
            shape,
            op: GraphOp::Add(a_id, b_id),
            requires_grad: req_grad,
        });

        id
    }

    /// Operasi Perkalian Elemen (Mul)
    pub fn mul(&mut self, a_id: usize, b_id: usize) -> usize {
        let len = self.nodes[a_id].data.len();
        let mut out_data = vec![0.0; len];
        for i in 0..len {
            out_data[i] = self.nodes[a_id].data[i] * self.nodes[b_id].data[i];
        }

        let id = self.nodes.len();
        let shape = self.nodes[a_id].shape.clone();
        let req_grad = self.nodes[a_id].requires_grad || self.nodes[b_id].requires_grad;

        self.nodes.push(AutoGradNode {
            id,
            data: out_data,
            grad: vec![0.0; len],
            shape,
            op: GraphOp::Mul(a_id, b_id),
            requires_grad: req_grad,
        });

        id
    }

    /// Aktivasi ReLU: f(x) = max(0, x)
    pub fn relu(&mut self, a_id: usize) -> usize {
        let len = self.nodes[a_id].data.len();
        let mut out_data = vec![0.0; len];
        for i in 0..len {
            out_data[i] = self.nodes[a_id].data[i].max(0.0);
        }

        let id = self.nodes.len();
        let shape = self.nodes[a_id].shape.clone();
        let req_grad = self.nodes[a_id].requires_grad;

        self.nodes.push(AutoGradNode {
            id,
            data: out_data,
            grad: vec![0.0; len],
            shape,
            op: GraphOp::ReLU(a_id),
            requires_grad: req_grad,
        });

        id
    }

    /// Mean Squared Error (MSE Loss)
    pub fn mse_loss(&mut self, pred_id: usize, target_id: usize) -> usize {
        let len = self.nodes[pred_id].data.len();
        let mut sum_sq = 0.0f32;
        for i in 0..len {
            let diff = self.nodes[pred_id].data[i] - self.nodes[target_id].data[i];
            sum_sq += diff * diff;
        }
        let loss = sum_sq / (len as f32);

        let id = self.nodes.len();
        self.nodes.push(AutoGradNode {
            id,
            data: vec![loss],
            grad: vec![0.0; 1],
            shape: vec![1],
            op: GraphOp::MSELoss(pred_id, target_id),
            requires_grad: true,
        });

        id
    }

    /// Reverse-Mode Automatic Differentiation (Backpropagation)
    pub fn backward(&mut self, root_id: usize) {
        // Set gradien awal untuk root = 1.0
        self.nodes[root_id].grad = vec![1.0; self.nodes[root_id].data.len()];

        // Backtrack dari root node ke leaf nodes (urutan terbalik)
        for i in (0..=root_id).rev() {
            let node = self.nodes[i].clone();
            if !node.requires_grad {
                continue;
            }

            match node.op {
                GraphOp::Leaf => {}
                GraphOp::Add(a_id, b_id) => {
                    for k in 0..node.grad.len() {
                        self.nodes[a_id].grad[k] += node.grad[k];
                        self.nodes[b_id].grad[k] += node.grad[k];
                    }
                }
                GraphOp::Sub(a_id, b_id) => {
                    for k in 0..node.grad.len() {
                        self.nodes[a_id].grad[k] += node.grad[k];
                        self.nodes[b_id].grad[k] -= node.grad[k];
                    }
                }
                GraphOp::Mul(a_id, b_id) => {
                    let a_data = self.nodes[a_id].data.clone();
                    let b_data = self.nodes[b_id].data.clone();
                    for k in 0..node.grad.len() {
                        self.nodes[a_id].grad[k] += node.grad[k] * b_data[k];
                        self.nodes[b_id].grad[k] += node.grad[k] * a_data[k];
                    }
                }
                GraphOp::ReLU(a_id) => {
                    let a_data = self.nodes[a_id].data.clone();
                    for k in 0..node.grad.len() {
                        let derivative = if a_data[k] > 0.0 { 1.0 } else { 0.0 };
                        self.nodes[a_id].grad[k] += node.grad[k] * derivative;
                    }
                }
                GraphOp::MSELoss(pred_id, target_id) => {
                    let pred_data = self.nodes[pred_id].data.clone();
                    let target_data = self.nodes[target_id].data.clone();
                    let n = pred_data.len() as f32;
                    for k in 0..pred_data.len() {
                        let d_loss_d_pred = (2.0 / n) * (pred_data[k] - target_data[k]);
                        self.nodes[pred_id].grad[k] += node.grad[0] * d_loss_d_pred;
                    }
                }
                _ => {}
            }
        }
    }

    /// Ambil nilai data dari node
    pub fn get_data(&self, id: usize) -> &[f32] {
        &self.nodes[id].data
    }

    /// Ambil nilai gradien dari node
    pub fn get_grad(&self, id: usize) -> &[f32] {
        &self.nodes[id].grad
    }

    /// Update parameter menggunakan Stochastic Gradient Descent (SGD)
    pub fn step_sgd(&mut self, param_ids: &[usize], learning_rate: f32) {
        for &id in param_ids {
            let len = self.nodes[id].data.len();
            for k in 0..len {
                self.nodes[id].data[k] -= learning_rate * self.nodes[id].grad[k];
                // Reset gradien
                self.nodes[id].grad[k] = 0.0;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_autograd_simple_linear_regression_step() {
        let mut graph = AutoGradEngine::new();

        // Parameter bobot: w = 2.0, x = 3.0, target = 10.0
        let w = graph.variable(vec![1], vec![2.0], true);
        let x = graph.variable(vec![1], vec![3.0], false);
        let target = graph.variable(vec![1], vec![10.0], false);

        // y = w * x
        let y = graph.mul(w, x);
        assert_eq!(graph.get_data(y)[0], 6.0);

        // loss = MSE(y, target) = (6 - 10)^2 = 16.0
        let loss = graph.mse_loss(y, target);
        assert_eq!(graph.get_data(loss)[0], 16.0);

        // Hitung gradien otomatis: d(loss)/d(w) = 2 * (y - target) * x = 2 * (6 - 10) * 3 = -24.0
        graph.backward(loss);
        assert_eq!(graph.get_grad(w)[0], -24.0);

        // SGD step dengan lr = 0.1: w_baru = 2.0 - 0.1 * (-24.0) = 4.4
        graph.step_sgd(&[w], 0.1);
        assert_eq!(graph.get_data(w)[0], 4.4);
    }
}
