use yaml_rust::YamlLoader;
use yaml_rust::yaml::Yaml;
use std::fs;
use std::collections::HashMap;

enum State {
	String(String),
	Int(i64)
}

/// Defines a persistant enviornmental condition created or "observed" by RPC calls
pub struct Variable {
	// Is this used in function input? It is used in output
	name: String,
	value: State 
}

impl Variable {
	fn new(config: Yaml) -> Variable {
		Variable {
			name: match config["name"] {
				Yaml::String(ref s) => s.to_string(),
				_ => panic!("Invalid config. String expected for name")
			},
			value: match config["value"] {
				Yaml::String(ref s) => State::String(s.to_string()),
				Yaml::Integer(ref i) => State::Int(*i),
				_ => panic!("Invalid config. String or Integer expected for value")
			}
		}
	}
}

pub struct Function {
	inputs: Vec<Variable>,
	outputs: Vec<Variable>,
}

impl Function {
	/// Creates a `Function` object from config
	fn new(config: Yaml) -> Function {
		for input in config {
			
		}
		Function {
			inputs: Vec::new(),
			outputs: Vec::new()
		}
	}
	/// Runs a function by computing its inputs and then its output
	fn run(inputs: HashMap<String, State>) -> HashMap<String, String> {
		HashMap::new()
	}
}

/// Defines a soap endpoint and all RPC calls
pub struct Endpoint {
	namespace: String,
	url: String,
	description: String,
	states: Vec<State>,
	functions: HashMap<String, Function>
}

impl Endpoint {
	fn new(config: &Yaml) -> Endpoint {
		let state_config = match config["variables"] {
			Yaml::Array(ref a) => a,
			_ => {panic!("Varibles are expected to be a vector")}
		};
		let mut states: Vec<State> = Vec::new();
		for state in state_config {
			states.push(match state {
				Yaml::String(ref s) => State::String(s.to_string()),
				Yaml::Integer(ref i) => State::Int(*i),
				_ => {panic!("Variables must either be strings or integers");}
			});
		}
		Endpoint {
			namespace: match config["namespace"] {
				Yaml::String(ref s) => s.to_string(),
				_ => panic!("Expected namespace to be string")
			},
			url: match config["url"] {
				Yaml::String(ref s) => s.to_string(),
				_ => panic!("Expected url to be string")
			},
			description: match config["description"] {
				Yaml::String(ref s) => s.to_string(),
				_ => panic!("Expected description to be string")
			},
			states: states,
			functions: HashMap::new()
		}
	}
}

/// Root root of the soap config file
pub struct Config {
	/// Path to the root description file
	description: String,
	headers: HashMap<String, String>,
	endpoints: Vec<Endpoint>
}

impl Config {
	pub fn new_from_path(path: &'static str) -> Config {
		let contents = fs::read_to_string(path)
			.expect("Something went wrong reading the file");
		let docs = YamlLoader::load_from_str(&contents[..]).unwrap();
		let config = &docs[0];
		let headers = match config["headers"] {
			Yaml::Hash(ref h) => h,
			_ => {panic!("Headers are expected to be a table")}
		};
		let mut header_hash: HashMap<String, String> = HashMap::new();
		for key in headers.keys() {
			let unwrapped_key = match key {
				Yaml::String(ref s) => s.to_string(),
				_ => {panic!("Header keys are expected to be strings")}
			};
			header_hash.insert(
				unwrapped_key,
				match headers.get(key) {
					Some(v) => match v {
						Yaml::String(ref s) => s.to_string(),
						_ => {panic!("Header values are expected to be strings")}
					},
					None => {panic!("Header values should be non empty")} // TODO: Is this true?
				}
			);
		}
		let mut endpoints: Vec<Endpoint> = Vec::new();
		let endpoint_config = match config["enpoints"] {
			Yaml::Array(ref a) => a,
			_ => {panic!("Enpoints are expected to be a vector")}
		};
		for endpoint in endpoint_config {
			endpoints.push(Endpoint::new(endpoint));
		}
		Config {
			description: match config["description"] {
				Yaml::String(ref s) => s.to_string(),
				_ => {panic!("Description is expected to be a string")},
			},
			headers: header_hash,
			endpoints: endpoints
		}
	}
}

