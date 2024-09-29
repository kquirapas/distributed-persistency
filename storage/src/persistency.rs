use super::models::Task;
use anyhow::Result;

pub trait Persistency: Send + Sync + 'static {
    async fn create_task(task: Task) -> Result<()>;
    // async fn update_task() -> Result<()>;
    // async fn delete_task() -> Result<()>;
}
