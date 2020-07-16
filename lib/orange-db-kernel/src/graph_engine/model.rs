use super::super::model::*;
use super::super::graph::traits::*;
use super::super::repository::graph_repository::GraphRepository;

use std::rc::Rc;
use std::cell::RefCell;

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
pub struct ProxyNodeId {
    mem_id: usize,
    store_id: u64,
    to_retrieve: bool,
}



impl MemGraphId for ProxyNodeId {
    fn get_index(&self) -> usize {
        self.mem_id
    }
}

impl ProxyNodeId {

    fn new(db_id: u64) -> Self {
        ProxyNodeId{mem_id: 0, store_id: db_id, to_retrieve: true}
    }

    fn get_store_id(&self) -> u64 {
        self.store_id
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
pub struct ProxyRelationshipId {
    mem_id: usize,
    store_id: u64,
}

impl MemGraphId for ProxyRelationshipId {
    fn get_index(&self) -> usize {
        self.mem_id
    }
}

impl ProxyRelationshipId {
    fn get_store_id(&self) -> u64 {
        self.store_id
    }
}

pub struct InnerNodeData<EID: MemGraphId> {
    first_outbound_edge: Option<EID>,
    first_inbound_edge: Option<EID>,
}

#[derive(Clone)]
pub struct InnerEdgeData<NID: MemGraphId, EID: MemGraphId> {
    pub source: NID,
    pub target: NID,
    pub next_outbound_edge: Option<EID>,
    pub next_inbound_edge: Option<EID>,
}

pub struct GraphProxy<'r> {
    nodes: Vec<Node>,
    relationships: Vec<Relationship>,
    vertices: Vec<InnerNodeData<ProxyRelationshipId>>,
    edges: Rc<RefCell<Vec<InnerEdgeData<ProxyNodeId, ProxyRelationshipId>>>>,
    repository: &'r GraphRepository,
    retrieved_nodes_ids: Vec<ProxyNodeId>,
}


impl <'r> GraphContainerTrait<ProxyNodeId, ProxyRelationshipId, Node, Relationship> for GraphProxy<'r> {

    fn get_node_mut(&mut self, id: &ProxyNodeId) -> &mut Node {
        &mut self.nodes[id.get_index()]
    }

    fn get_relationship_mut(&mut self, id: &ProxyRelationshipId) -> &mut Relationship {
        &mut self.relationships[id.get_index()]
    }

    fn get_node_ref(&self, id: &ProxyNodeId) -> &Node {
        &self.nodes[id.get_index()]
    }

    fn get_relationship_ref(&self, id: &ProxyRelationshipId) -> &Relationship {
        &self.relationships[id.get_index()]
    }

}

pub struct InEdges {
    edges: Rc<RefCell<Vec<InnerEdgeData<ProxyNodeId, ProxyRelationshipId>>>>,
    current_edge_index: Option<ProxyRelationshipId>,
}

impl Iterator for InEdges {
    type Item = ProxyRelationshipId;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current_edge_index {
            None => None,
            Some(edge_index) => {
                let edge = &self.edges.borrow()[edge_index.get_index()];
                let curr_edge_index = self.current_edge_index;
                self.current_edge_index = edge.next_inbound_edge;
                curr_edge_index
            }
        }
    }
}

pub struct OutEdges {
    edges: Rc<RefCell<Vec<InnerEdgeData<ProxyNodeId, ProxyRelationshipId>>>>,
    current_edge_index: Option<ProxyRelationshipId>,
}

impl Iterator for OutEdges {
    type Item = ProxyRelationshipId;

    fn next(&mut self) -> Option<ProxyRelationshipId> {
        match self.current_edge_index {
            None => None,
            Some(edge_index) => {
                let edge = &self.edges.borrow()[edge_index.get_index()];
                let curr_edge_index = self.current_edge_index;
                self.current_edge_index = edge.next_outbound_edge;
                curr_edge_index
            }
        }
    }
}

impl <'g, 'r> GraphIteratorTrait<ProxyNodeId, ProxyRelationshipId> for &'g mut GraphProxy<'r> {
    type OutIt = OutEdges;
    type InIt = InEdges;
    fn out_edges(&self, source: &ProxyNodeId) -> OutEdges {
        let first_outbound_edge = self.vertices[source.get_index()].first_outbound_edge;
        OutEdges{ edges: self.edges.clone(), current_edge_index: first_outbound_edge }
    }

    fn in_edges(&self, target: &ProxyNodeId) -> Self::InIt {
        let first_inbound_edge = self.vertices[target.get_index()].first_inbound_edge;
        InEdges{ edges: self.edges.clone(), current_edge_index: first_inbound_edge }
    }
}

impl <'r> GraphIteratorTrait<ProxyNodeId, ProxyRelationshipId> for GraphProxy<'r> {
    type OutIt = OutEdges;
    type InIt = InEdges;
    fn out_edges(&self, source: &ProxyNodeId) -> OutEdges {
        let first_outbound_edge = self.vertices[source.get_index()].first_outbound_edge;
        OutEdges{ edges: self.edges.clone(), current_edge_index: first_outbound_edge }
    }

    fn in_edges(&self, target: &ProxyNodeId) -> Self::InIt {
        let first_inbound_edge = self.vertices[target.get_index()].first_inbound_edge;
        InEdges{ edges: self.edges.clone(), current_edge_index: first_inbound_edge }
    }
}


impl <'r> GraphTrait<ProxyNodeId, ProxyRelationshipId> for GraphProxy<'r> {
    fn get_source_index(&self, edge_index: &ProxyRelationshipId) -> ProxyNodeId {
        self.edges.borrow()[edge_index.get_index()].source
    }
    fn get_target_index(&self, edge_index: &ProxyRelationshipId) -> ProxyNodeId {
        self.edges.borrow()[edge_index.get_index()].target
    }
    fn nodes_len(&self) -> usize {
        self.retrieved_nodes_ids.len()
    }
    fn edges_len(&self) -> usize {
        self.relationships.len()
    }
    
    fn get_nodes_ids(&self) -> Vec<ProxyNodeId> {
        Vec::new()
    }

    fn in_degree(&self, node: &ProxyNodeId) -> usize {
        0//self.in_edges(node).count()
    }
    fn out_degree(&self, node: &ProxyNodeId) -> usize {
        0//self.out_edges(node).count()
    }

}

impl <'g> GrowableGraph<ProxyNodeId> for GraphProxy<'g> {
    
    fn retrieve_out_edges(&mut self, source: &ProxyNodeId) {
        
    }

    fn retrieve_in_edges(&mut self, target: &ProxyNodeId) {
        
    }
}


fn retrieve_db_nodes_ids(repository: &mut GraphRepository, labels: &Vec<String>) -> Vec<ProxyNodeId> {
    let db_node_ids = repository.fetch_nodes_ids_with_labels(labels);
    let mut res = Vec::new();
    for id in db_node_ids {
        res.push(ProxyNodeId::new(id))
    }
    res
}

impl <'r> GraphProxy<'r> {
    pub fn new(repo: &'r mut GraphRepository, labels: Vec<String>) -> Self {
        let ids = retrieve_db_nodes_ids(repo, &labels);
        GraphProxy{repository: repo, nodes: Vec::new(),
            relationships: Vec::new(),
            retrieved_nodes_ids: ids, vertices: Vec::new(), edges: Rc::new(RefCell::new(Vec::new()))}
    }
}




#[cfg(test)]
mod test_cache_model {
    use super::*;
    fn test_add_prop_graphs() {
    }

}