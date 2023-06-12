use schemars::gen::SchemaSettings;
use schemars::schema_for;
use serde_yaml;
use ya_client_model::net::*;

fn main() {
    let mut net_api_schema: serde_json::Value =
        serde_yaml::from_str(include_str!("../../specs/net-api.yaml")).unwrap();

    let objects = net_api_schema
        .get_mut("components")
        .unwrap()
        .get_mut("schemas")
        .unwrap()
        .as_object_mut()
        .unwrap();

    let settings = SchemaSettings::draft07().with(|s| {
        s.option_nullable = true;
        s.option_add_null_type = false;
        s.inline_subschemas = false;
    });
    let mut gen = settings.into_generator();

    let mut show_diff_for = |name, schema, skip| {
        let lhs: serde_json::Value = objects.remove(name).unwrap().clone();
        let rhs: serde_json::Value = serde_json::to_value(schema).unwrap();
        let df = json_schema_diff::diff(lhs, rhs.clone()).unwrap();

        if df.is_empty() || skip {
            eprintln!("{}", serde_yaml::to_string(&rhs).unwrap());
            eprintln!("{} is equal", name);
        } else {
            eprintln!("# {}", name);
            eprintln!("{}", serde_yaml::to_string(&rhs).unwrap());
            eprintln!("# {} changes", name);
            for change in df {
                eprintln!("{:20} {:?}", change.path, change.change);
            }
        }
        //eprintln!("L: {}", serde_json::to_string_pretty(&lhs).unwrap());
    };
    show_diff_for("Node", gen.root_schema_for::<Node>(), false);
    show_diff_for("Address", gen.root_schema_for::<Address>(), false);
    show_diff_for("Connection", gen.root_schema_for::<Connection>(), false);
    show_diff_for("Network", gen.root_schema_for::<Network>(), false);
    show_diff_for("Protocol", gen.root_schema_for::<Protocol>(), true);
    show_diff_for("Proxy", gen.root_schema_for::<Proxy>(), false);

    for key in objects.keys() {
        eprintln!(
            "show_diff_for(\"{}\", gen.root_schema_for::<{}>(), false);",
            key, key
        );
    }

    assert!(objects.is_empty())
}
