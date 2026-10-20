use sqlx::postgres::PgPoolOptions;
use board_card_service::board::{get_all_boards, create_board, rename_board_title, delete_board};


// Execute here for stub only
#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    const DATABASE: &str = "postgres://sggen:Ipg@qn242526@localhost:5432/kanbandb";
    let pool = PgPoolOptions::new().max_connections(5).connect(DATABASE).await?;
    println!("Successfully connected to {DATABASE}!");

    match get_all_boards(&pool).await {
        Ok(boards) => {
            for board in &boards {
                println!("ID: {}, Title: {}, Created At: {}", board.id, board.title, board.created_at);
            }
        }
        Err(e) => eprintln!("Error fetching boards: {}", e),
    }

    // match create_board(&pool, 8, "New Board").await {
    //     Ok(created_id) => println!("Created board with ID: {}", created_id),
    //     Err(e) => eprintln!("Error creating board: {}", e),
    // }

    // match rename_board_title(&pool, 1, "Updated Test Board with Time NOW").await {
    //     Ok(updated_id) => println!("Updated board with ID: {}", updated_id),
    //     Err(e) => eprintln!("Error updating board: {}", e),
    // }

    // match delete_board(&pool, 8).await {
    //     Ok(deleted_id) => println!("Deleted board with ID: {}", deleted_id),
    //     Err(e) => eprintln!("Error deleting board: {}", e),
    // }

    Ok(())
}