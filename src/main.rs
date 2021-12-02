// osmtograph extract graph object from OpenStreetMap data
// Copyright (C) 2021 Jules Azad EMERY a.k.a. ethicnology
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
#![feature(destructuring_assignment)]

mod geo;
mod heuristics;
mod metrics;
mod openstreetmap;
mod utils;

use crate::Node;
use geo::*;
use heuristics::*;
use metrics::*;
use openstreetmap::*;
use structopt::StructOpt;
use utils::*;

#[derive(StructOpt)]
#[structopt(name = "osmtograph")]
enum OsmToGraph {
    /// Format OSM filtered by way file to put one xml element by line
    Format,
    /// Extract all nodes data : node_id␟key␟value␟key␟value…
    Nodes,
    /// Extract links from ways nodes : node_id␟node_id␟way_id
    Links,
    /// Extract ways data : way_id␟key␟value␟key␟value…
    Ways,
    /// Apply heuristics.
    Heuristics {
        /// Delta is expressed in meters.
        #[structopt(short, long)]
        delta: f32,
    },
}

fn main() {
    match OsmToGraph::from_args() {
        OsmToGraph::Format => format_xml(),
        OsmToGraph::Nodes => extract_nodes(),
        OsmToGraph::Links => extract_links(),
        OsmToGraph::Ways => extract_ways(),
        OsmToGraph::Heuristics { delta } => {
            let (mut nodes, mut links) = load_graph();
            metrics(&nodes, &links, ("0", delta.to_string()));
            (nodes, links) = remove_degree_two_nodes(nodes, links);
            metrics(&nodes, &links, ("1", delta.to_string()));
            (nodes, links) = remove_under_delta_nodes(nodes, links, delta);
            metrics(&nodes, &links, ("2", delta.to_string()));
            (nodes, links) = remove_under_delta_links(nodes, links, delta);
            metrics(&nodes, &links, ("3", delta.to_string()));
            (nodes, links) = bfs_connected_components_distribution_and_largest(&nodes, &links);
            metrics(&nodes, &links, ("4", delta.to_string()));
            (nodes, links) = discretize(nodes, links, delta);
            metrics(&nodes, &links, ("5", delta.to_string()));
            print_graph(&nodes, &links);
        }
    }
}
