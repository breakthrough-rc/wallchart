// EXAMPLE CODE
// use db::schema::{
//     worksites,
// };
// use diesel::prelude::Associations;
// use diesel::{
//     prelude::{Identifiable, Queryable},
//     AsChangeset, Insertable, Selectable,
// };
//
// #[derive(Queryable, Selectable, Insertable, AsChangeset, Identifiable, PartialEq, Debug)]
// #[diesel(table_name = worksites)]
// pub struct WorksiteRecord {
//     pub id: String,
//     pub name: String,
// }
//
// #[derive(
//     Queryable, Selectable, Insertable, AsChangeset, Identifiable, Associations, Debug, PartialEq,
// )]
// #[diesel(belongs_to(WorksiteRecord, foreign_key=worksite_id))]
// #[diesel(table_name = locations)]
// pub struct LocationRecord {
//     pub id: String,
//     pub name: String,
//     pub worksite_id: String,
// }
