use juniper::{graphql_object, Executor, FieldResult, GraphQLObject};
use rusqlite::{params, Connection};

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
#[graphql(description = "A page of games")]
struct Games {
    infos: Infos,
    results: Vec<Game>,
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

pub struct Context {
    pub connection: Connection,
}

impl juniper::Context for Context {}

pub struct Query;

fn game(executor: &&Executor<'_, Context>, id: String) -> FieldResult<Option<Game>> {
    let connection = &executor.context().connection;

    log::info!("{}", id);
    let mut statement = match connection.prepare("SELECT * FROM game where id = ?1 limit 1") {
        Ok(stmt) => stmt,
        Err(e) => panic!("{}", e),
    };

    Ok(statement
        .query_map(params![id], |row| {
            Ok(Game {
                id: row.get(0)?,
                name: row.get(1)?,
                genres: Vec::new(),
                studios: Vec::new(),
                editors: Vec::new(),
                platform: Vec::new(),
                publication_date: row.get(2)?,
            })
        })
        .and_then(|mut games| games.next().transpose())
        .unwrap_or(None))
}

fn games(
    executor: &&Executor<'_, Context>,
    page: Option<i32>,
    genre: Option<String>,
    platform: Option<String>,
    studio: Option<String>,
) -> FieldResult<Games> {
    let current_page = page.unwrap_or(0);

    let connection = &executor.context().connection;

    let mut statement = match connection.prepare(
        "
    SELECT g.* FROM game g
    LEFT JOIN game_studio_link gsl ON g.id = gsl.game_id
    LEFT JOIN studio s ON s.id = gsl.studio_id 
    WHERE (s.name = ?1 OR ?1 IS NULL) 
    LIMIT ?2 OFFSET ?3;",
    ) {
        Ok(stmt) => stmt,
        Err(e) => panic!("{}", e),
    };

    let rows = statement.query_map(params![studio, 20, current_page * 20], |row| {
        Ok(Game {
            id: row.get(0)?,
            name: row.get(1)?,
            genres: Vec::new(),
            studios: Vec::new(),
            editors: Vec::new(),
            platform: Vec::new(),
            publication_date: row.get(2)?,
        })
    })?;

    let mut games: Vec<Game> = Vec::new();

    for row in rows {
        if let Ok(game) = row {
            games.push(game)
        }
    }

    Ok(Games {
        infos: Infos {
            count: games.len() as i32,
            pages: current_page,
            next_page: current_page + if games.len() == 20 { 1 } else { 0 },
            previous_page: if current_page == 0 {
                0
            } else {
                current_page - 1
            },
        },
        results: games,
    })
}

graphql_object!(Query: Context |&self| {
    field apiVersion() -> &str {
        "0.1"
    }
    field game(&executor, id: String) -> FieldResult<Option<Game>> {
        game(executor, id)
    }
    field games(&executor,
    page: Option<i32>,
    genre: Option<String>,
    platform: Option<String>,
    studio: Option<String>,
) -> FieldResult<Games> {
        games(executor, page, genre, platform, studio)
    }
});

pub struct Mutation;

graphql_object!(Mutation: Context |&self| {
    field fake() -> FieldResult<bool> {
        Ok(true)
    }
});

pub type Schema = juniper::RootNode<'static, Query, Mutation>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, Mutation {})
}
