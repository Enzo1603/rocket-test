use rocket::form::FromFormField;

#[derive(sqlx::Type, Debug, FromFormField)]
#[repr(i32)]
pub enum PostType {
    Text = 0,
    Photo = 1,
    Video = 2,
}
