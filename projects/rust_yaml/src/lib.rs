use log::{debug, info};
use serde::{Deserialize, Serialize};
use yaml_rust2::{YamlEmitter, YamlLoader};

/**
 * &strのyaml定義から連想配列形式を作成 & 連想配列からyaml定義のString作成
 */
pub fn run_example() {
    debug!("run_example");
    let s = "
foo:
    - list1
    - list2
bar:
    - 1
    - 2.0
";
    let docs = YamlLoader::load_from_str(s).unwrap();

    // Multi document support, doc is a yaml::Yaml
    let doc = &docs[0];

    // Debug support
    info!("{:?}", doc);

    // Index access for map & array
    assert_eq!(doc["foo"][0].as_str().unwrap(), "list1");
    assert_eq!(doc["bar"][1].as_f64().unwrap(), 2.0);

    // Array/map-like accesses are checked and won't panic.
    // They will return `BadValue` if the access is invalid.
    assert!(doc["INVALID_KEY"][100].is_badvalue());

    // Dump the YAML object
    let mut out_str = String::new();
    {
        let mut emitter = YamlEmitter::new(&mut out_str);
        emitter.dump(doc).unwrap(); // dump the YAML object to a String
    }
    info!("{}", out_str);
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
enum Shape {
    Rectangle { width: u32, height: u32 },
    Circle { radius: f64 },
    Triangle { base: u32, height: u32 },
}

#[derive(Debug, Serialize, Deserialize)]
struct Point {
    x: f64,
    y: f64,
    shapes: Vec<Shape>,
    age1: Option<u8>,
    age2: Option<u8>,
    is_active: bool,
}

/**
 * yaml定義をstructに入れたい
 */
pub fn run_yaml_struct() {
    debug!("run_yaml_struct");
    let point = Point {
        x: 1.0,
        y: 2.0,
        shapes: vec![
            Shape::Rectangle {
                width: 100,
                height: 120,
            },
            Shape::Circle { radius: 100.0 },
            Shape::Triangle {
                base: 130,
                height: 140,
            },
        ],
        age1: None,
        age2: Some(9),
        is_active: false,
    };
    info!("target object: {:?}", &point);

    // Serialize to YAML
    let yaml = serde_yml::to_string(&point).unwrap();
    info!("Serialize to yaml string: {}", &yaml);

    // Deserialize from YAML
    let deserialized_point: Point = serde_yml::from_str(&yaml).unwrap();
    info!("deserialize to yaml string: {:?}", &deserialized_point);
}
