mod geo;
mod graph;
mod heuristics;
mod openstreetmap;
mod overpass;

use clap::Parser;
use geo::*;
use graph::*;
use heuristics::*;
use openstreetmap::*;
use overpass::*;

#[derive(Parser)]
#[clap(author, version, bin_name = "osmtograph")]
enum OsmToGraph {
    Download {
        /// Select any available cities/areas in overpass-api: Pantin, Damas, Mexico, Paris, London, Tokyo, Moscow…
        #[clap(short, long)]
        city: String,
        /// ⚠With caution⚠: please learn overpass QL. City variable is stored in 'area'.
        #[clap(short, long, default_value = "(way(area)[highway]; ); (._;>;);")]
        overpassql: String,
    },
    Format,
    Extract {
        /// Specify a custom separator such as space: -s ' '. Beware that data already contains the dot '.' and comma ','
        #[clap(short, long, default_value_t = '␟')]
        separator: char,
    },
    Heuristics {
        /// Specify a custom separator such as space: -s ' '. Beware that data already contains the dot '.' and comma ','
        #[clap(short, long, default_value_t = '␟')]
        separator: char,
    },
}

fn main() {
    match OsmToGraph::parse() {
        OsmToGraph::Download { city, overpassql } => download_map(city, overpassql).unwrap(),
        OsmToGraph::Format => format_xml(),
        OsmToGraph::Extract { separator } => extract(separator),
        OsmToGraph::Heuristics { separator } => {
            //let mut graph = Graph::from("2576426859␟48.8275541␟2.3489099\n2576426853␟48.8274352␟2.348721\n3761637489␟48.8275453␟2.348698\n2576426856␟48.8275026␟2.3485468\n3758221284␟48.8273411␟2.3486982\n92192237␟48.8275872␟2.3490245\n3761637486␟48.8275249␟2.348704\n3761637488␟48.8275416␟2.3486683\n1829061602␟48.8275089␟2.3484223\n3758221301␟48.8275751␟2.3489308\n2268836829␟48.8276001␟2.3486802\n2576426850␟48.8274242␟2.3486471\n3761637482␟48.8274512␟2.3486719\n2576426858␟48.8275464␟2.3489207\n6400885441␟48.8274338␟2.3488187\n3758221295␟48.8275185␟2.3484976\n1852590201␟48.8276523␟2.3494784\n2576426854␟48.8274412␟2.3487844\n2576426851␟48.8274323␟2.3487423\n3758221292␟48.8274025␟2.3486929\n1829061614␟48.8273732␟2.3487375\n2576426855␟48.827493␟2.3485442\n2576426852␟48.8274347␟2.3487671\n3761637490␟48.8275499␟2.348735\n3761637496␟48.8278544␟2.3473522\n2576426847␟48.8273391␟2.3487858\n3758221301␟92192237\n2576426855␟3761637482\n1829061614␟3758221284\n1829061602␟3761637496\n1852590201␟92192237\n1829061614␟6400885441\n2576426853␟3761637482\n2576426851␟2576426852\n2576426850␟3761637482\n2576426855␟2576426856\n3758221301␟3761637490\n3761637482␟3761637486\n6400885441␟92192237\n3761637488␟3761637489\n1829061614␟3758221292\n1829061602␟2576426850\n3758221295␟3761637488\n3761637486␟3761637489\n2576426853␟3758221292\n1829061614␟2576426847\n3761637489␟3761637490\n2576426858␟2576426859\n2576426856␟3761637486\n2576426851␟2576426853\n2576426859␟3761637486\n1829061602␟3758221295\n2576426852␟2576426854\n2268836829␟3761637489\n2576426850␟3758221292\n2576426854␟2576426858", separator);
            let mut graph = Graph::load(separator);
            graph = remove_degree_two_nodes(graph);
            graph.show(separator);
        }
    }
}
