pub mod traits;
pub mod container;
use self::traits::*;

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
pub struct EdgeIndex {
    index: usize,
}

impl EdgeIndex {
    pub fn new(index: usize) -> Self {
        EdgeIndex {index: index}
    }
}

impl traits::MemGraphId for EdgeIndex {
    fn get_index(&self) -> usize {
        self.index
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
pub struct NodeIndex {
    index: usize,
}

impl NodeIndex {
    pub fn new(index: usize) -> Self {
        NodeIndex {index: index}
    }
}

impl traits::MemGraphId for NodeIndex {
    fn get_index(&self) -> usize {
        self.index
    }
}

pub struct VertexData<EID: MemGraphId> {
    pub first_outbound_edge: Option<EID>,
    pub first_inbound_edge: Option<EID>,
}

impl <EID: MemGraphId + Copy> VertexData<EID> {
    pub fn get_first_outbound_edge(&self) -> Option<EID> {
        self.first_outbound_edge
    }
    pub fn get_first_inbound_edge(&self) -> Option<EID> {
        self.first_inbound_edge
    }
}

#[derive(Clone)]
pub struct EdgeData<NID: MemGraphId, EID: MemGraphId> {
    pub source: NID,
    pub target: NID,
    pub next_outbound_edge: Option<EID>,
    pub next_inbound_edge: Option<EID>,
}

impl <NID: MemGraphId + Copy, EID: MemGraphId + Copy> EdgeData<NID, EID> {
    pub fn get_source(&self) -> NID {
        self.source
    }
    pub fn get_target(&self) -> NID {
        self.target
    }

    pub fn get_next_outbound_edge(&self) -> Option<EID> {
        self.next_outbound_edge
    }
    pub fn get_next_inbound_edge(&self) ->  Option<EID> {
        self.next_inbound_edge
    }
}

pub struct Graph {
    nodes: Vec<VertexData<EdgeIndex>>,
    edges: Vec<EdgeData<NodeIndex, EdgeIndex>>,
}


pub struct OutEdges {
    edges: Vec<EdgeData<NodeIndex, EdgeIndex>>,
    current_edge_index: Option<EdgeIndex>,
}

impl Iterator for OutEdges {
    type Item = EdgeIndex;

    fn next(&mut self) -> Option<EdgeIndex> {
        match self.current_edge_index {
            None => None,
            Some(edge_index) => {
                let edge = &self.edges[edge_index.get_index()];
                let curr_edge_index = self.current_edge_index;
                self.current_edge_index = edge.next_outbound_edge;
                curr_edge_index
            }
        }
    }
}


pub struct InEdges {
    edges: Vec<EdgeData<NodeIndex, EdgeIndex>>,
    current_edge_index: Option<EdgeIndex>,
}

impl Iterator for InEdges {
    type Item = EdgeIndex;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current_edge_index {
            None => None,
            Some(edge_index) => {
                let edge = &self.edges[edge_index.get_index()];
                let curr_edge_index = self.current_edge_index;
                self.current_edge_index = edge.next_inbound_edge;
                curr_edge_index
            }
        }
    }
}

impl <'g> GraphTrait<NodeIndex, EdgeIndex> for Graph {
    type OutIt = OutEdges;
    type InIt = InEdges;
    fn out_edges(&self, source: &NodeIndex) -> Self::OutIt {
        let first_outbound_edge = self.nodes[source.get_index()].first_outbound_edge;
        OutEdges{ edges: self.edges.clone(), current_edge_index: first_outbound_edge }
    }

    fn in_edges(&self, target: &NodeIndex) -> InEdges {
        let first_inbound_edge = self.nodes[target.get_index()].first_inbound_edge;
        InEdges{ edges: self.edges.clone(), current_edge_index: first_inbound_edge }
    }

    fn get_source_index(&self, edge_index: &EdgeIndex) -> &NodeIndex {
        &self.edges[edge_index.get_index()].source
    }
    fn get_target_index(&self, edge_index: &EdgeIndex) -> &NodeIndex {
        &self.edges[edge_index.get_index()].target
    }

    fn nodes_len(&self) -> usize {
        self.nodes.len()
    }

    fn edges_len(&self) -> usize {
        self.edges.len()
    }

    fn get_nodes_ids(&self) -> Vec<NodeIndex> {
        (0..self.nodes_len()).map(NodeIndex::new).collect()
    }
    
    fn in_degree(&self, node: &NodeIndex) -> usize {
        self.in_edges(node).count()
    }
    fn out_degree(&self, node: &NodeIndex) -> usize {
        self.out_edges(node).count()
    }
}
impl Graph {
    pub fn new() -> Self {
        Graph{ nodes: Vec::new(), edges: Vec::new() }
    }

    pub fn add_vertex(&mut self) -> NodeIndex {
        let index = self.nodes.len();
        self.nodes.push(VertexData::<EdgeIndex>{first_outbound_edge: None, first_inbound_edge: None});
        NodeIndex::new(index)
    }

    pub fn get_vertex(&self, id: NodeIndex) -> &VertexData<EdgeIndex> {
        &self.nodes[id.get_index()]
    }
    pub fn get_edge(&self, id: EdgeIndex) -> &EdgeData<NodeIndex, EdgeIndex> {
        &self.edges[id.get_index()]
    }

    pub fn add_edge(&mut self, source: NodeIndex, target: NodeIndex) -> EdgeIndex {
        let index = self.edges.len();
        {
            let source_data = &self.nodes[source.get_index()];
            let target_data = &self.nodes[target.get_index()];
            self.edges.push(EdgeData{source: source, target: target,
                 next_inbound_edge: target_data.first_inbound_edge, 
                 next_outbound_edge: source_data.first_outbound_edge});
        }
        
        let ms = &mut self.nodes[source.get_index()];
        ms.first_outbound_edge = Some(EdgeIndex::new(index));
        let mt = &mut self.nodes[target.get_index()];
        mt.first_inbound_edge = Some(EdgeIndex::new(index));
        EdgeIndex::new(index)
    }

    pub fn get_nodes(&self) -> &Vec<VertexData<EdgeIndex>> {
        &self.nodes
    }
    pub fn get_edges(&self) -> &Vec<EdgeData<NodeIndex, EdgeIndex>> {
        &self.edges
    }
}

#[cfg(test)]
mod test_graph {
    use super::*;
    #[test]
    fn test_small_graph_it() {
        let mut graph = Graph::new();
        let n0 = graph.add_vertex();
        let n1 = graph.add_vertex();
        let n2 = graph.add_vertex();

        let e0 = graph.add_edge(n0, n1);
        let e1 = graph.add_edge(n1, n2);
        let e2 = graph.add_edge(n0, n2);

        let ed0 = graph.get_edge(e0);
        assert_eq!(ed0.source, n0);
        assert_eq!(ed0.target, n1);
        assert_eq!(ed0.next_outbound_edge, None);
        
        let nd0 = graph.get_vertex(n0);
        assert_eq!(nd0.first_outbound_edge, Some(e2));

        let ed2 = graph.get_edge(e2);
        assert_eq!(ed2.source, n0);
        assert_eq!(ed2.target, n2);
        assert_eq!(ed2.next_outbound_edge, Some(e0));

    }
}