use juniper::{graphql_object, FieldResult, GraphQLObject};

#[derive(GraphQLObject)]
#[graphql(description = "A page of objext informations")]
struct Infos {
    count: i32,
    pages: i32,
    next_page: i32,
    previous_page: i32,
}

#[derive(GraphQLObject)]
#[graphql(description = "An Edior entrprise")]
struct Editor {
    id: String,
    name: String,
    games: Vec<Game>,
}

#[derive(GraphQLObject)]
#[graphql(description = "A studio entreprise")]
struct Studio {
    id: String,
    name: String,
    games: Vec<Game>,
}

#[derive(GraphQLObject)]
#[graphql(description = "A page of editors")]
struct Editors {
    infos: Infos,
    results: Vec<Editor>,
}

#[derive(GraphQLObject)]
#[graphql(description = "A page of studios")]
struct Studios {
    infos: Infos,
    results: Vec<Studio>,
}

#[derive(GraphQLObject)]
#[graphql(description = "Representation of a video game")]
struct Game {
    id: String,
    name: String,
    genres: Vec<String>,
    publication_date: i32,
    editors: Vec<Editor>,
    studios: Vec<Studio>,
    platform: Vec<String>,
}

pub struct Context {}

impl juniper::Context for Context {}

pub struct Query;

graphql_object!(Query: Context |&self| {
    field apiVersion() -> &str {
    "0.1"
    }
    field game(&executor, id: String) -> FieldResult<Game> {
        println!("{}", id);
        let game: Game = Game {
        id:String::from("1"),
        name: String::from("Toto gros"),
        genres: Vec::from([String::from("Ghibluh")]),
        publication_date: 33,
        editors: Vec::new(),
        studios: Vec::new(),
        platform: Vec::from([String::from("ps2")]),
        };

        Ok(game)
    }
});

pub struct Mutation;

graphql_object!(Mutation: Context |&self| {

});

pub type Schema = juniper::RootNode<'static, Query, Mutation>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, Mutation {})
}
