use juniper::Variables;

mod schema;

fn main() {
    let ctx = schema::Context {};
    let schema = schema::create_schema();

    let (res, _errors) = juniper::execute(
        "query { game(id: \"123\") { id, name}}",
        None,
        &schema,
        &Variables::new(),
        &ctx,
    )
    .unwrap();
    let test = res
        .as_object_value()
        .unwrap()
        .get_field_value("game")
        .unwrap()
        .as_object_value()
        .unwrap()
        .get_field_value("name")
        .unwrap()
        .as_string_value()
        .unwrap();

    print!("{}", test);
}
