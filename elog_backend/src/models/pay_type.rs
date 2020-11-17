#[derive(Queryable)]
pub struct PayType {
    pub id: u8,
    pub name: String,
    pub description: String,
    pub status: bool
}
