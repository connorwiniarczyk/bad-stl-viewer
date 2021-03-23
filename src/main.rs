extern crate kiss3d;
extern crate nalgebra as na;

use kiss3d::light::Light;
use kiss3d::resource::Mesh;
use kiss3d::window::Window;

use kiss3d::nalgebra::{Point3, UnitQuaternion, Vector3};

use std::cell::RefCell;
use std::rc::Rc;

use stl_io::IndexedMesh;
use std::convert::TryInto;
use std::fs::OpenOptions;
use std::env;


/// Use the stl_io library to read an stl file with the given path and return
/// its mesh as stl_io's IndexedMesh type
fn read(path: &str) -> stl_io::IndexedMesh {
	let mut file = OpenOptions::new().read(true).open(&path).unwrap();
	let stl = stl_io::read_stl(&mut file).unwrap();
	return stl
}

/// The kiss3d library and stl_io library both define types for storing a 3D
/// mesh of triangles which are very similar, but slightly different. This 
/// function converts stl_io's Mesh type into kiss3d's Mesh type by looping
/// over and converting its components. 
fn to_kiss3d_mesh(mesh: &stl_io::IndexedMesh) -> Mesh {
	
	// Copy vertices by converting Vec<f32> to Point3<f32> for each vertex
	let mut vertices: Vec<Point3<f32>> = vec![];
	for vertex in &mesh.vertices {
		let point = Point3::new(vertex[0], vertex[1], vertex[2]);
		vertices.push(point);
	}

	// Copy faces and normals from stl_io's IndexedTriangle type
	let mut faces: Vec<Point3<u16>> = vec![];
	let mut normals: Vec<Vector3<f32>> = vec![];
	for face in &mesh.faces {

		// TODO: converting between usize and u16 should be better. Ideally I 
		// think something other than u16s should be used to index the verticces,
		// since this puts a hard cap on the number of vertices a mesh can have.
		let first: u16 = face.vertices[0].try_into().unwrap();
		let second: u16 = face.vertices[1].try_into().unwrap();
		let third: u16 = face.vertices[2].try_into().unwrap();

		let point = Point3::new(first, second, third);
		faces.push(point);

		// copy normals
		// TODO: Normals from the stl_io object do not seem to copy correctly,
		// It looks like it may be copying normals from the wrong faces, or 
		// mixing up their dimensions somehow.
		let normal = Vector3::new(face.normal[0], face.normal[1], face.normal[2]);
		normals.push(normal);
	}

	// TODO: replace first 'None' with normals from the stl_io Mesh
	Mesh::new(vertices, faces, None, None, false)
}

/// Builds a new kiss3d window with a given title and renders a mesh to it.
/// Basically copied exactly from the kiss3d custom mesh example
/// https://github.com/sebcrozet/kiss3d/blob/master/examples/custom_mesh.rs
fn render(title: &str, model: Mesh) {

	let mesh = Rc::new(RefCell::new(model));
	let mut window = Window::new(title);
	let mut c = window.add_mesh(mesh, Vector3::new(1.0, 1.0, 1.0));

	c.set_color(1.0, 1.0, 1.0);

	c.enable_backface_culling(true);
	window.set_light(Light::StickToCamera);

	// rotate the model 90 degrees to keep the up axis consistent
	let axis_adjust = UnitQuaternion::from_axis_angle(&Vector3::x_axis(), -1.5707);
	c.prepend_to_local_rotation(&axis_adjust);

	// apply a slow spin animation
	let slow_spin = UnitQuaternion::from_axis_angle(&Vector3::z_axis(), 0.0005);
	while window.render() {
		c.prepend_to_local_rotation(&slow_spin);
	}
}

fn main() {

	// Get cli arguments to determine path of stl file to display.
	// If 1 or more arguments are given, use the first argument as the path,
	// if 0 arguments are given, use a default path
	// TODO: some other options would be kinda cool
	let args: Vec<String> = env::args().collect();
	let default_file = "~/example.stl";
	let path = match args.len() {
		1 => {
			println!("");
			println!("Please specify an STL file");
			println!("Since none was specified, bad-stl will default to ~/example.stl");
			println!("but this is unlikely to exist, and the program will most likely panic.");
			println!("---------------------");
			println!("usage: bad-stl <file>");
			println!("");
			println!("");
			default_file
		},
		2 => &args[1],
		_ => &args[1],
	};

	// read the file and convert it into a mesh
	let stl_io_mesh = read(path);
	let kiss3d_mesh = to_kiss3d_mesh(&stl_io_mesh);

	// render the mesh to a new window
	render(&path, kiss3d_mesh);
}
