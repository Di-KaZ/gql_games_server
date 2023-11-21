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

fn get_editors_from_game_id(connection: &Connection, id: &String) -> Vec<Editor> {
    let mut statement = match connection.prepare(
        "
        SELECT * FROM editor 
        inner join game_editor_link gel on gel.game_id = ?1 and gel.editor_id = id
        ",
    ) {
        Ok(stmt) => stmt,
        Err(e) => panic!("{}", e),
    };

    let rows = statement
        .query_map(params![id], |row| {
            Ok(Editor {
                id: row.get(0)?,
                name: row.get(1)?,
                games: Vec::new(),
            })
        })
        // TODO: understand more rust error management for now it will do got no time
        .unwrap();

    let mut editors: Vec<Editor> = Vec::new();

    for row in rows {
        if let Ok(editor) = row {
            editors.push(editor)
        }
    }

    editors
}

fn get_studios_from_game_id(connection: &Connection, id: &String) -> Vec<Studio> {
    let mut statement = match connection.prepare(
        "
        SELECT * FROM studio 
        inner join game_studio_link gsl on gsl.game_id = ?1 and gsl.studio_id = id
        ",
    ) {
        Ok(stmt) => stmt,
        Err(e) => panic!("{}", e),
    };

    let rows = statement
        .query_map(params![id], |row| {
            Ok(Studio {
                id: row.get(0)?,
                name: row.get(1)?,
                games: Vec::new(),
            })
        })
        // TODO: understand more rust error management for now it will do got no time
        .unwrap();

    let mut studios: Vec<Studio> = Vec::new();

    for row in rows {
        if let Ok(editor) = row {
            studios.push(editor)
        }
    }

    studios
}

fn game(executor: &&Executor<'_, Context>, id: String) -> FieldResult<Option<Game>> {
    let connection = &executor.context().connection;

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
                studios: get_studios_from_game_id(connection, &id),
                editors: get_editors_from_game_id(connection, &id),
                platform: Vec::new(),
                publication_date: row.get(2)?,
            })
        })
        .and_then(|mut games| games.next().transpose())
        .unwrap_or(None))
}

fn studio(executor: &&Executor<'_, Context>, id: String) -> FieldResult<Option<Studio>> {
    let connection = &executor.context().connection;

    log::info!("{}", id);
    let mut statement = match connection.prepare("SELECT * FROM studio where id = ?1 limit 1") {
        Ok(stmt) => stmt,
        Err(e) => panic!("{}", e),
    };

    Ok(statement
        .query_map(params![id], |row| {
            Ok(Studio {
                id: row.get(0)?,
                name: row.get(1)?,
                games: Vec::new(),
            })
        })
        .and_then(|mut games| games.next().transpose())
        .unwrap_or(None))
}

fn editor(executor: &&Executor<'_, Context>, id: String) -> FieldResult<Option<Editor>> {
    let connection = &executor.context().connection;

    log::info!("{}", id);
    let mut statement = match connection.prepare("SELECT * FROM editor where id = ?1 limit 1") {
        Ok(stmt) => stmt,
        Err(e) => panic!("{}", e),
    };

    Ok(statement
        .query_map(params![id], |row| {
            Ok(Editor {
                id: row.get(0)?,
                name: row.get(1)?,
                games: Vec::new(),
            })
        })
        .and_then(|mut games| games.next().transpose())
        .unwrap_or(None))
}

fn studios(executor: &&Executor<'_, Context>, page: Option<i32>) -> FieldResult<Studios> {
    let current_page = page.unwrap_or(0);

    let connection = &executor.context().connection;

    let mut statement = match connection.prepare(
        "
    SELECT * FROM studio 
    LIMIT ?1 OFFSET ?2;",
    ) {
        Ok(stmt) => stmt,
        Err(e) => panic!("{}", e),
    };

    let rows = statement.query_map(params![20, current_page * 20], |row| {
        Ok(Studio {
            id: row.get(0)?,
            name: row.get(1)?,
            games: Vec::new(),
        })
    })?;

    let mut editors: Vec<Studio> = Vec::new();

    for row in rows {
        if let Ok(editor) = row {
            editors.push(editor)
        }
    }

    Ok(Studios {
        infos: Infos {
            count: editors.len() as i32,
            pages: current_page,
            next_page: current_page + if editors.len() == 20 { 1 } else { 0 },
            previous_page: if current_page == 0 {
                0
            } else {
                current_page - 1
            },
        },
        results: editors,
    })
}

fn editors(executor: &&Executor<'_, Context>, page: Option<i32>) -> FieldResult<Editors> {
    let current_page = page.unwrap_or(0);

    let connection = &executor.context().connection;

    let mut statement = match connection.prepare(
        "
    SELECT * FROM editor
    LIMIT ?1 OFFSET ?2;",
    ) {
        Ok(stmt) => stmt,
        Err(e) => panic!("{}", e),
    };

    let rows = statement.query_map(params![20, current_page * 20], |row| {
        Ok(Editor {
            id: row.get(0)?,
            name: row.get(1)?,
            games: Vec::new(),
        })
    })?;

    let mut editors: Vec<Editor> = Vec::new();

    for row in rows {
        if let Ok(editor) = row {
            editors.push(editor)
        }
    }

    Ok(Editors {
        infos: Infos {
            count: editors.len() as i32,
            pages: current_page,
            next_page: current_page + if editors.len() == 20 { 1 } else { 0 },
            previous_page: if current_page == 0 {
                0
            } else {
                current_page - 1
            },
        },
        results: editors,
    })
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
        let id: String = row.get(0)?;
        Ok(Game {
            id: id.to_owned(),
            name: row.get(1)?,
            genres: Vec::new(),
            studios: get_studios_from_game_id(connection, &id),
            editors: get_editors_from_game_id(connection, &id),
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

pub struct Context {
    pub connection: Connection,
}

impl juniper::Context for Context {}

pub struct Query;

graphql_object!(Query: Context |&self| {
    field apiVersion() -> &str {
        "0.1"
    }
    field game(&executor, id: String) -> FieldResult<Option<Game>> {
        game(executor, id)
    }
    field studio(&executor, id: String) -> FieldResult<Option<Studio>> {
        studio(executor, id)
    }
    field editor(&executor, id: String) -> FieldResult<Option<Editor>> {
        editor(executor, id)
    }
    field editors(
        &executor,
        page: Option<i32>,
    ) -> FieldResult<Editors> {
        editors(executor, page)
    }
    field studios(
        &executor,
        page: Option<i32>,
    ) -> FieldResult<Studios> {
        studios(executor, page)
    }
    field games(
        &executor,
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
