#[derive(Debug, Queryable, LabelledGeneric)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Debug, LabelledGeneric)]
pub struct NoIdPost {
    pub title: String,
    pub body: String,
    pub published: bool,
}
