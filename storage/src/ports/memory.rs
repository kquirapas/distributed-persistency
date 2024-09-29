use crate::persistency::Persistency;

pub struct Memory;

impl Persistency for Memory {
    async fn create_task(task: Task) -> Result<()> {
        Ok(())
    }
}
