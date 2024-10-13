use random_number::random;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub(crate) struct NodeConfig {
    pub btc: Network,
}

#[derive(Deserialize, Debug)]
pub struct Network {
    #[allow(dead_code)]
    pub devnet: Vec<NodeSource>,
    #[allow(dead_code)]
    pub testnet: Vec<NodeSource>,
    pub mainnet: Vec<NodeSource>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct NodeSource {
    #[allow(dead_code)]
    pub name: String,
    pub url: String,
}

pub fn get_one_node_from_sources(sources: Vec<NodeSource>) -> NodeSource {
    sources[random!(..sources.len())].clone()
}

#[test]
fn get_one_node_from_sources_test() {
    use crate::CONFIG;
    let sources = &CONFIG.node.btc.testnet;
    let one_rand_node = &sources[random!(..sources.len())];
    println!("{:?}", one_rand_node);
}
