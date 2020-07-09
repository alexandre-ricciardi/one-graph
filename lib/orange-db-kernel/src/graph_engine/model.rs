use super::super::model::*;
use super::super::graph::traits::{GraphTrait, GraphContainerTrait, MemGraphId};
use super::super::repository::graph_repository::GraphRepository;

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

pub struct InnerEdgeData<NID: MemGraphId, EID: MemGraphId> {
    pub source: NID,
    pub target: NID,
    pub next_outbound_edge: Option<EID>,
    pub next_inbound_edge: Option<EID>,
}


pub struct InnerGraph<'r> {
    nodes: Vec<InnerNodeData<ProxyRelationshipId>>,
    edges: Vec<InnerEdgeData<ProxyNodeId, ProxyRelationshipId>>,
    repository: &'r GraphRepository,    
}

pub struct GraphProxy<'r> {
    nodes: Vec<Node>,
    labels: Vec<String>,
    relationships: Vec<Relationship>,
    graph: InnerGraph<'r>,
    repository: &'r GraphRepository,
}


impl <'g> GraphContainerTrait<'g, ProxyNodeId, ProxyRelationshipId, Node, Relationship> for GraphProxy<'g> {

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

pub struct InEdges<'g> {
    graph: &'g InnerGraph<'g>,
    current_edge_index: Option<ProxyRelationshipId>,
}

impl <'graph> Iterator for InEdges<'graph> {
    type Item = ProxyRelationshipId;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current_edge_index {
            None => None,
            Some(edge_index) => {
                let edge = &self.graph.edges[edge_index.get_index()];
                let curr_edge_index = self.current_edge_index;
                self.current_edge_index = edge.next_inbound_edge;
                curr_edge_index
            }
        }
    }
}

pub struct OutEdges<'g> {
    graph: &'g InnerGraph<'g>,
    current_edge_index: Option<ProxyRelationshipId>,
}

impl <'g> Iterator for OutEdges<'g> {
    type Item = ProxyRelationshipId;

    fn next(&mut self) -> Option<ProxyRelationshipId> {
        match self.current_edge_index {
            None => None,
            Some(edge_index) => {
                let edge = &self.graph.edges[edge_index.get_index()];
                let curr_edge_index = self.current_edge_index;
                self.current_edge_index = edge.next_outbound_edge;
                curr_edge_index
            }
        }
    }
}


impl <'g> GraphTrait<'g, ProxyNodeId, ProxyRelationshipId> for InnerGraph<'g> {
    type OutIt = OutEdges<'g>;
    type InIt = InEdges<'g>;
    fn out_edges(&self, source: &ProxyNodeId) -> OutEdges {
        let first_outbound_edge = self.nodes[source.get_index()].first_outbound_edge;
        OutEdges{ graph: self, current_edge_index: first_outbound_edge }
    }

    fn in_edges(&self, target: &ProxyNodeId) -> InEdges {
        let first_inbound_edge = self.nodes[target.get_index()].first_inbound_edge;
        InEdges{ graph: self, current_edge_index: first_inbound_edge }
    }

    fn get_source_index(&self, edge_index: &ProxyRelationshipId) -> &ProxyNodeId {
        &self.edges[edge_index.get_index()].source
    }
    fn get_target_index(&self, edge_index: &ProxyRelationshipId) -> &ProxyNodeId {
        &self.edges[edge_index.get_index()].target
    }

    fn nodes_len(&self) -> usize {
        self.nodes.len()
    }

    fn edges_len(&self) -> usize {
        self.edges.len()
    }
 
    fn get_nodes_ids(&self) -> Vec<ProxyNodeId> {
        
        Vec::new()//(0..self.nodes_len()).map(ProxyNodeId::new).collect()
    }
    
    fn in_degree(&self, node: &ProxyNodeId) -> usize {
        self.in_edges(node).count()
    }
    fn out_degree(&self, node: &ProxyNodeId) -> usize {
        self.out_edges(node).count()
    }
}

impl <'g> GraphTrait<'g, ProxyNodeId, ProxyRelationshipId> for GraphProxy<'g> {
    type OutIt = OutEdges<'g>;
    type InIt = InEdges<'g>;
    fn out_edges(&'g self, source: &ProxyNodeId) -> OutEdges {
        self.graph.out_edges(source)
    }

    fn in_edges(&'g self, target: &ProxyNodeId) -> Self::InIt {
        self.graph.in_edges(target)
    }
    fn get_source_index(&self, edge_index: &ProxyRelationshipId) -> &ProxyNodeId {
        &self.graph.edges[edge_index.get_index()].source
    }
    fn get_target_index(&self, edge_index: &ProxyRelationshipId) -> &ProxyNodeId {
        &self.graph.edges[edge_index.get_index()].target
    }
    fn nodes_len(&self) -> usize {
        self.nodes.len()
    }
    fn edges_len(&self) -> usize {
        self.relationships.len()
    }
    
    fn get_nodes_ids(&self) -> Vec<ProxyNodeId> {
        Vec::new()
    }

    fn in_degree(&self, node: &ProxyNodeId) -> usize {
        self.in_edges(node).count()
    }
    fn out_degree(&self, node: &ProxyNodeId) -> usize {
        self.out_edges(node).count()
    }
}

impl <'r> InnerGraph<'r> {
    fn new(repo: &'r GraphRepository) -> Self {
        InnerGraph{nodes: Vec::new(), edges: Vec::new(), repository: repo}
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
        GraphProxy{repository: repo, nodes: Vec::new(), relationships: Vec::new(), graph: InnerGraph::new(repo), labels: labels}
    }
}




#[cfg(test)]
mod test_cache_model {
    use super::*;
    fn test_add_prop_graphs() {
    }

}