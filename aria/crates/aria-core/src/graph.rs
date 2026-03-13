use std::path::{Path, PathBuf};
use std::collections::{HashMap, HashSet, VecDeque};
use crate::manifest::{Manifest, ManifestBody, Layer};
use crate::checker::{Diagnostic, Severity, CheckResult};

/// A node in the Semantic Graph, corresponding to one manifest.
#[derive(Debug, Clone)]
pub struct GraphNode {
    pub id: String,
    pub file: PathBuf,
    pub layer: Layer,
    pub manifest: ManifestBody,
}

/// The Semantic Graph: nodes are manifests, edges are declared dependencies.
pub struct SemanticGraph {
    /// Map from ARU id → node
    nodes: HashMap<String, GraphNode>,
    /// Adjacency list: id → list of dependency ids
    edges: HashMap<String, Vec<String>>,
}

impl SemanticGraph {
    /// Build a SemanticGraph from a set of loaded manifests.
    pub fn build(manifests: &[(PathBuf, Manifest)]) -> Self {
        let mut nodes = HashMap::new();
        let mut edges: HashMap<String, Vec<String>> = HashMap::new();

        for (path, m) in manifests {
            let id = m.manifest.id.clone();
            let deps: Vec<String> = m.manifest.dependencies.iter().map(|d| d.id.clone()).collect();
            nodes.insert(id.clone(), GraphNode {
                id: id.clone(),
                file: path.clone(),
                layer: m.manifest.layer.declared.clone(),
                manifest: m.manifest.clone(),
            });
            edges.insert(id, deps);
        }

        SemanticGraph { nodes, edges }
    }

    /// Returns a reference to a node by ARU id.
    pub fn get_node(&self, id: &str) -> Option<&GraphNode> {
        self.nodes.get(id)
    }

    /// Returns all nodes in the graph.
    pub fn nodes(&self) -> impl Iterator<Item = &GraphNode> {
        self.nodes.values()
    }

    /// Returns all direct dependencies of an ARU.
    pub fn dependencies(&self, id: &str) -> &[String] {
        self.edges.get(id).map(|v| v.as_slice()).unwrap_or(&[])
    }

    /// Returns all ARUs that transitively depend on `target_id`, in topological order.
    /// Groups results by layer.
    pub fn transitive_dependents(&self, target_id: &str) -> Vec<String> {
        // Build reverse edges: id → list of ARUs that depend on it
        let mut reverse: HashMap<String, Vec<String>> = HashMap::new();
        for (from, deps) in &self.edges {
            for dep in deps {
                reverse.entry(dep.clone()).or_default().push(from.clone());
            }
        }

        // BFS from target
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut result = vec![];

        if let Some(dependents) = reverse.get(target_id) {
            for d in dependents {
                queue.push_back(d.clone());
            }
        }

        while let Some(id) = queue.pop_front() {
            if visited.contains(&id) { continue; }
            visited.insert(id.clone());
            result.push(id.clone());
            if let Some(dependents) = reverse.get(&id) {
                for d in dependents {
                    if !visited.contains(d) {
                        queue.push_back(d.clone());
                    }
                }
            }
        }

        result
    }

    /// Detect cycles in the dependency graph using Kahn's algorithm (topological sort).
    /// Returns diagnostics for each cycle detected.
    pub fn check_cycles(&self) -> CheckResult {
        let mut diagnostics = vec![];

        // Build in-degree map
        let mut in_degree: HashMap<&str, usize> = HashMap::new();
        for id in self.nodes.keys() {
            in_degree.entry(id.as_str()).or_insert(0);
        }
        for deps in self.edges.values() {
            for dep in deps {
                in_degree.entry(dep.as_str()).or_insert(0);
            }
        }
        for deps in self.edges.values() {
            for dep in deps {
                *in_degree.entry(dep.as_str()).or_insert(0) += 1;
            }
        }

        // Kahn's: enqueue all zero-in-degree nodes
        let mut queue: VecDeque<&str> = in_degree
            .iter()
            .filter(|(_, &deg)| deg == 0)
            .map(|(&id, _)| id)
            .collect();

        let mut processed = 0;
        while let Some(id) = queue.pop_front() {
            processed += 1;
            if let Some(deps) = self.edges.get(id) {
                for dep in deps {
                    let deg = in_degree.entry(dep.as_str()).or_insert(0);
                    if *deg > 0 {
                        *deg -= 1;
                        if *deg == 0 {
                            queue.push_back(dep.as_str());
                        }
                    }
                }
            }
        }

        if processed < self.nodes.len() {
            // There are nodes not reached — they form cycles
            for (id, &deg) in &in_degree {
                if deg > 0 {
                    if let Some(node) = self.nodes.get(*id) {
                        diagnostics.push(Diagnostic::error(
                            node.file.clone(),
                            0, 0,
                            format!("Cycle detected in dependency graph involving ARU '{}'", id),
                        ));
                    }
                }
            }
        }

        diagnostics
    }

    /// Check for cross-domain dependency violations.
    /// L1–L3 ARUs must not depend on ARUs from different domain boundaries.
    pub fn check_cross_domain_dependencies(&self) -> CheckResult {
        let mut diagnostics = vec![];

        for node in self.nodes.values() {
            let layer_num = node.layer.numeric();
            if layer_num < 1 || layer_num > 3 { continue; }

            let node_domain = extract_domain(&node.id);

            for dep_id in self.dependencies(&node.id) {
                let dep_domain = extract_domain(dep_id);
                if dep_domain != node_domain {
                    // Check if the dependency is an L0 type (those can cross domains)
                    let dep_layer_ok = self.nodes.get(dep_id.as_str())
                        .map(|n| n.layer == Layer::L0)
                        .unwrap_or(false);

                    if !dep_layer_ok {
                        diagnostics.push(Diagnostic::error(
                            node.file.clone(),
                            0, 0,
                            format!(
                                "Cross-domain dependency violation: '{}' (domain: {}) depends on '{}' (domain: {})",
                                node.id, node_domain, dep_id, dep_domain
                            ),
                        ));
                    }
                }
            }
        }

        diagnostics
    }
}

fn extract_domain(id: &str) -> &str {
    id.split('.').next().unwrap_or(id)
}
