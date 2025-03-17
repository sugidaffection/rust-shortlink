use crate::prelude::*;
use tera::Tera;

#[derive(Clone)]
pub struct AppData {
    pub tera: Tera,
    pub pool: DbPool,
}

impl AppData {
    pub fn new(template_path: Option<&str>) -> Self {
        let tera = Tera::new(template_path.unwrap_or("templates/**/*"))
            .expect("Template folder not found");
        let pool: DbPool = create_pool(None);

        Self {
            tera: tera,
            pool: pool,
        }
    }
}
