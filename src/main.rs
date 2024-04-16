use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::{thread, time};

use raylib::{misc::AsF32, prelude::*};
use rand::distributions::{Distribution, Uniform};
const WIDTH: f32 = 800.0;
const HEIGHT: f32 = 800.0;
const WINDOWWIDTH: f32 = 1600.0;
const WINDOWHEIGHT: f32 = 1600.0;

const MARGIN: f32 = 1.8 ;
mod   rect_utils;

#[derive(PartialEq, Clone, Debug)]
struct  Vertex {
    id: String,
    position: (f32, f32), // Vertex position (x, y)
    forces: (f32, f32),

}
#[derive(Clone, Copy, Debug)]
struct Edge {
    source: usize,
    target: usize,
}

// Define a graph representation (e.g., adjacency list)
#[derive(Clone)]
struct Graph {
    vertices: Vec<Vertex>,
    edges: Vec<Edge>,
}

// impl  Graph {
//     fn new() -> Graph {       
//         Graph { vertices: vec![], edges: vec![] }
//     }

// }


// Initialize Vertex positions (randomly)
fn initialize_positions(graph: &mut Graph) {

    // Set up random normal
    let mut rng = rand::thread_rng();
    let x_rand = Uniform::from(-0.5*WIDTH..0.5*WIDTH);
    let y_rand = Uniform::from(-0.5*HEIGHT..0.5*HEIGHT);
    // let x_rand = Uniform::from(0.0..WIDTH as f32) ;
    // let y_rand = Uniform::from(0.0..HEIGHT as f32);

    // use iterator to loop through all the vertices and assign random x,y positions
    let _vertices: Vec<()> = graph.vertices.iter_mut().map(|node|{
        node.position = (x_rand.sample(&mut rng), y_rand.sample(&mut rng));
    }).collect();
    
}

fn initialize_forces(graph: &mut Graph) {
    let _vertices: Vec<()> = graph.vertices.iter_mut().map(|node|{
        node.forces = (0.0,0.0);
    }).collect();   
}

// function that mocks Epsilon: used to avoid divided by zero.
fn clamp_epsilon(dist: &mut f32) -> f32 {
 
    let epsilon = 0.05;

    dist.max(epsilon)
}

fn compute_attraction_forces(graph: &mut Graph, k_factor: f32) {
   for edge in &graph.edges {
    // Extract the positions
    let mut distance = ((graph.vertices[edge.source].position.0 - graph.vertices[edge.target].position.0).powf(2.0) 
            +  (graph.vertices[edge.source].position.1 - graph.vertices[edge.target].position.1).powf(2.0)).sqrt();
    
    distance = clamp_epsilon(&mut distance);

    let fx = distance * (graph.vertices[edge.target].position.0 - graph.vertices[edge.source].position.0)/k_factor;
    let fy = distance * (graph.vertices[edge.target].position.1 - graph.vertices[edge.source].position.1)/k_factor;

    graph.vertices[edge.source].forces.0 += fx ;
    graph.vertices[edge.source].forces.1 += fy ;

    graph.vertices[edge.target].forces.0 -= fx ;
    graph.vertices[edge.target].forces.1 -= fy ;   
    

   }  

}

fn compute_repulsion_forces(graph: &mut Graph, k_factor: f32){
    for i in 0..graph.vertices.len() {
        for j in 0..graph.vertices.len() {
            if i != j {
                let mut distance = ((graph.vertices[i].position.0 - graph.vertices[j].position.0).powf(2.0) 
                                        +  (graph.vertices[i].position.1 - graph.vertices[j].position.1).powf(2.0)).sqrt();
                distance = clamp_epsilon(&mut distance);

                let fx = k_factor *k_factor * (graph.vertices[j].position.0 - graph.vertices[i].position.0) / distance.powf(2.0);
                let fy = k_factor *k_factor * (graph.vertices[j].position.1 - graph.vertices[i].position.1) / distance.powf(2.0);

                graph.vertices[i].forces.0 -= fx/2.0 ;
                graph.vertices[i].forces.1 -= fy/2.0 ;
                graph.vertices[j].forces.0 += fx/2.0 ;
                graph.vertices[j].forces.1 += fy/2.0 ;

                // println!("id: {},fx: {}, fy: {}",graph.vertices[i].id, graph.vertices[i].forces.0,graph.vertices[i].forces.1);
                // println!("id: {},fx: {}, fy: {}",graph.vertices[j].id, graph.vertices[j].forces.0,graph.vertices[j].forces.1);

                // println!("---------------------------------------------------------------");

                //u_node.forces.0 += fx/2.0 ;
                //u_node.forces.1 += fy/2.0 ;
            
                //v_node.forces.0 += fx/2.0 ;
                //v_node.forces.1 += fy/2.0 ;                 

            }
        }
    }

}

// Calculate attractive and repulsive forces
fn calculate_gravity_forces(graph: &mut Graph, k_factor: f32) {

    for i in  0..graph.vertices.len() {
        let mut distance = ((graph.vertices[i].position.0).powf(2.0) + (graph.vertices[i].position.1).powf(2.0)).sqrt();
        distance = clamp_epsilon(&mut distance);

        let fx = distance * graph.vertices[i].position.0 / k_factor ;
        let fy = distance * graph.vertices[i].position.1 / k_factor ;
        
        graph.vertices[i].forces.0 -= fx/10.0 ;
        graph.vertices[i].forces.1 -= fy/10.0 ;

        

    }
}

// Update Vertex positions
fn update_positions(graph: &mut Graph, temp: f32, margin: f32) {

    for u in 0..graph.vertices.len() {
       let modulo = ((graph.vertices[u].forces.0).powf(2.0) + (graph.vertices[u].forces.1).powf(2.0)).sqrt() ;

        if modulo > temp {
            graph.vertices[u].forces.0 *= temp / modulo;
            graph.vertices[u].forces.1 *= temp / modulo;

        }
    
       graph.vertices[u].position.0 += graph.vertices[u].forces.0;
       graph.vertices[u].position.1 += graph.vertices[u].forces.1;

        let cte = margin-0.1;
        if graph.vertices[u].position.0 < -cte*WIDTH{
            graph.vertices[u].position.0 = -cte*WIDTH;
        }
        if graph.vertices[u].position.1 < -cte*WIDTH {
            graph.vertices[u].position.1 = -cte*WIDTH;
        }
        if graph.vertices[u].position.0 > (cte*WIDTH){
            graph.vertices[u].position.0 = cte*WIDTH;
        }
        if graph.vertices[u].position.1 > (cte*WIDTH){
            graph.vertices[u].position.1 = cte*WIDTH;
        }


    }

}

fn draw(drawing:  &mut RaylibDrawHandle,location: Rectangle, graph: &Graph) {
    drawing.draw_rectangle_rec(location, Color::GOLD);
    for edge in &graph.edges {
        let x: Vector2 = (graph.vertices[edge.source].position.0+WIDTH , graph.vertices[edge.source].position.1+HEIGHT).into();
        let y: Vector2 = (graph.vertices[edge.target].position.0+WIDTH , graph.vertices[edge.target].position.1+HEIGHT).into();

        drawing.draw_line_ex(x, y, 4.0,Color::BLUE);
        drawing.draw_circle_v(x, 15.0, Color::RED);
        drawing.draw_circle_v(y, 15.0, Color::GREEN);


        let ten_millis = time::Duration::from_millis(10);
        
        //thread::sleep(ten_millis);
    }

}


pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_file(filename: &str) -> Option<Graph> {
    let mut vertices:Vec<Vertex> = Vec::new();
    let mut edges:Vec<(&str,&str)> = Vec::new();
    let mut edges_idx:Vec<Edge> = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        
        let  lines_vec: Vec<String> = lines.into_iter().map(|line| line.expect("Something went wrong while reading the file.")).collect();
        let nb_vertices: usize = lines_vec[0].trim().parse().expect("Expect a integer");

        for i in 1..=nb_vertices{
            vertices.push(Vertex { 
                id: lines_vec[i].trim().to_string(),
                position: (0.0,0.0),
                forces: (0.0,0.0),
                });
        }
        for i in (nb_vertices+1)..lines_vec.len(){
            let edg = lines_vec[i].trim().split_once(' ').expect("Expect 2 vertices");
            edges.push(edg);

        }
        //println!("vertices: {:?}", vertices);

        //println!("edges: {:?}", edges);

        for edge in edges {
            let  source:usize;
            let  target:usize;
            match vertices.iter().position(|so | so.id == edge.0 ) {
                Some(pos) => {source = pos ;} 
                None => {panic!("Invalid edge: {:?}", edge)}
            }
            match vertices.iter().position(|so | so.id == edge.1 ){
                Some(pos) => {target = pos ;}
                None => {panic!("Invalid edge: {:?}", edge)}
            }
            edges_idx.push(Edge {source: source,target: target});
        }

       //println!("edges with indices: {:?}", edges_idx);
    
        return Some(Graph{vertices: vertices, edges: edges_idx});
    }

    return None;

}

fn main() {

    let (mut raylb, thread) = raylib::init()
    .size(WINDOWWIDTH as i32, WINDOWHEIGHT as i32 )
    .title("Fruchterman-Reingold Graph")
    .build();

    raylb.set_target_fps(60);

    let window = Rectangle::new(0.as_f32(), 0.as_f32(), WINDOWWIDTH.as_f32(), WINDOWHEIGHT.as_f32());

    let margin: f32 = 0.90;
    let grid_rect = rect_utils::center_rect(window, margin, margin).unwrap();

    let mut graph: Graph; // Initialize the graph
    

    match parse_file("./grafos/petersen.txt"){
        Some(graph_f) => { graph = graph_f ;}
        None => {panic!(" Error parsing the file");}
    }

    println!("Graph has: {} vertices and {} edges", graph.vertices.len(), graph.edges.len());
    println!("{:?}",graph.edges);

    const NUM_ITERATIONS: usize = 400;
    const COOLING_FACTOR: f32 = 0.95;
    const FORCE_CONSTANT: f32 = 1.3;
    const INITIAL_TEMP: f32 = 100.0;
    const MIN_TEMP: f32 = 0.05 ;
    let k_factor = (FORCE_CONSTANT * (WIDTH*HEIGHT)/(graph.vertices.len() as f32)).sqrt();

    initialize_positions(&mut graph);
    let mut temp = INITIAL_TEMP;
    let margin = MARGIN;
    initialize_forces(&mut graph);

    println!("Initial positions:");
    for node in &graph.vertices {
        println!("Node {0: <7} X position: {1: >10.3} Y position: {2: >10.3}", node.id, node.position.0, node.position.1)
    }

    let mut i:usize = 0;
    while !raylb.window_should_close() {
        let mut drawing = raylb.begin_drawing(&thread);
         
        drawing.clear_background(Color::LIGHTGRAY);

        while i < NUM_ITERATIONS {
            initialize_forces(&mut graph);

            compute_attraction_forces(&mut graph, k_factor);

            compute_repulsion_forces(&mut graph, k_factor);

            calculate_gravity_forces(&mut graph, k_factor);

            update_positions(&mut graph, temp, margin);

            temp *= COOLING_FACTOR;

            if temp < MIN_TEMP {
                println!("Graph cooled completely");

                println!("Number of iterations: {}", i);
                i = NUM_ITERATIONS;
                break;

            } else {
                i += 1;
            }

        }


        if temp >= MIN_TEMP {
            println!("Iterations ran short");

            println!("Number of iterations: {}", NUM_ITERATIONS) ;
        }


        draw(&mut drawing, grid_rect, &graph);

        drawing.draw_fps(5, 5)

    }

    println!("Final positions:");
    for node in &graph.vertices {
        println!("Node {0: <7} X position: {1: >10.3} Y position: {2: >10.3}", node.id, node.position.0, node.position.1)
    }
}
