use rusqlite::Connection;
use rusqlite::Result;

pub fn init_db() -> Result<Connection> {
    let connection = Connection::open("games.db")?;

    return match connection.execute_batch(
        "
    CREATE TABLE IF NOT EXISTS game (
        id TEXT PRIMARY KEY,
        name TEXT NOT NULL,
        publication_date int NOT Null
    );
    
    CREATE TABLE IF NOT EXISTS studio (
        id TEXT PRIMARY KEY,
        name TEXT NOT NULL
    );
    
    CREATE TABLE IF NOT EXISTS editor (
        id TEXT PRIMARY KEY,
        name TEXT NOT NULL
    );
    
    CREATE TABLE IF NOT EXISTS game_editor_link (
        game_id TEXT,
        editor_id TEXT,
        FOREIGN KEY (game_id) REFERENCES game (id),
        FOREIGN KEY (editor_id) REFERENCES editor (id),
        PRIMARY KEY (game_id, editor_id)
    );
    
    CREATE TABLE IF NOT EXISTS game_studio_link (
        game_id TEXT,
        studio_id TEXT,
        FOREIGN KEY (game_id) REFERENCES game (id),
        FOREIGN KEY (studio_id) REFERENCES studio (id),
        PRIMARY KEY (game_id, studio_id)
    );

    INSERT INTO studio (id, name) VALUES
        ('550e8400-e29b-41d4-a716-446655440000', 'Ubisoft'),
        ('550e8400-e29b-41d4-a716-446655440001', 'Electronic Arts'),
        ('550e8400-e29b-41d4-a716-446655440002', 'Rockstar Games');
    
    INSERT INTO editor (id, name) VALUES
        ('550e8400-e29b-41d4-a716-446655440003', 'John Doe'),
        ('550e8400-e29b-41d4-a716-446655440004', 'Jane Smith'),
        ('550e8400-e29b-41d4-a716-446655440005', 'Alex Johnson'),
        ('550e8400-e29b-41d4-a716-446655440006', 'Emily Davis');
    
    INSERT INTO game (id, name, publication_date) VALUES 
        ('550e8400-e29b-41d4-a716-446655440007', 'Assassins Creed Valhalla', 12),
        ('550e8400-e29b-41d4-a716-446655440008', 'FIFA 22', 11),
        ('550e8400-e29b-41d4-a716-446655440009', 'Grand Theft Auto V', 14);
    
    INSERT INTO game_editor_link (game_id, editor_id) VALUES
        ('550e8400-e29b-41d4-a716-446655440007', '550e8400-e29b-41d4-a716-446655440003'),
        ('550e8400-e29b-41d4-a716-446655440007', '550e8400-e29b-41d4-a716-446655440004'),
        ('550e8400-e29b-41d4-a716-446655440008', '550e8400-e29b-41d4-a716-446655440004'),
        ('550e8400-e29b-41d4-a716-446655440009', '550e8400-e29b-41d4-a716-446655440005');
    
    INSERT INTO game_studio_link (game_id, studio_id) VALUES
        ('550e8400-e29b-41d4-a716-446655440007', '550e8400-e29b-41d4-a716-446655440000'),
        ('550e8400-e29b-41d4-a716-446655440008', '550e8400-e29b-41d4-a716-446655440001'),
        ('550e8400-e29b-41d4-a716-446655440009', '550e8400-e29b-41d4-a716-446655440002');
    ",
    ) {
        Err(err) => Err(err),
        Ok(_) => Ok(connection),
    };
}
pub fn get_connection() -> Result<Connection> {
    Connection::open("games.db")
}
