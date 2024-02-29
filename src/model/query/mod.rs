use crate::Result as LocoResult;
use sea_orm::{
    sea_query::{IntoCondition, Order},
    ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryOrder, Value,
};
use serde::{Deserialize, Serialize};
pub mod date_range;
pub mod pagination;

pub struct ConditionBuilder {
    condition: Condition,
    pagination_query: pagination::PaginationQuery,
}
/// Enum representing sorting directions, with serialization and deserialization support.
#[derive(Debug, Deserialize, Serialize)]
pub enum SortDirection {
    #[serde(rename = "desc")]
    Desc,
    #[serde(rename = "asc")]
    Asc,
}

impl SortDirection {
    /// Returns the corresponding `Order` enum variant based on the current `SortDirection`.
    const fn order(&self) -> Order {
        match self {
            Self::Desc => Order::Desc,
            Self::Asc => Order::Asc,
        }
    }
}

#[must_use]
pub fn condition() -> ConditionBuilder {
    ConditionBuilder {
        condition: Condition::all(),
        pagination_query: pagination::PaginationQuery::default(),
    }
}

#[must_use]
pub const fn with(
    condition: Condition,
    pagination_query: pagination::PaginationQuery,
) -> ConditionBuilder {
    ConditionBuilder {
        condition,
        pagination_query,
    }
}

/// Builder query condition
///
/// # Examples
/// ```
/// use loco_rs::tests_cfg::db::*;
/// use sea_orm::{EntityTrait, QueryFilter, QuerySelect, QueryTrait};
/// use loco_rs::prelude::{model::query::*, *};
/// let date = chrono::NaiveDateTime::parse_from_str("2024-03-01 22:10:57", "%Y-%m-%d %H:%M:%S").unwrap();
///
/// let query_str = test_db::Entity::find()
///         .select_only()
///         .column(test_db::Column::Id)
///         .filter(condition().date_range(test_db::Column::CreatedAt).from(&date).build().like(test_db::Column::Name, "%lo").build())
///         .build(sea_orm::DatabaseBackend::Postgres)
///         .to_string();
///
///     assert_eq!(
///         query_str,
///         "SELECT \"loco\".\"id\" FROM \"loco\" WHERE \"loco\".\"created_at\" > '2024-03-01 22:10:57' AND \"loco\".\"name\" LIKE '%lo'"
///     );
/// ````
impl ConditionBuilder {
    /// where condition the given column equals the given value
    ///
    /// # Examples
    /// ```
    /// use loco_rs::tests_cfg::db::*;
    /// use sea_orm::{EntityTrait, QueryFilter, QuerySelect, QueryTrait};
    /// use loco_rs::prelude::{model::query::*, *};
    ///
    /// let query_str = test_db::Entity::find()
    ///         .select_only()
    ///         .column(test_db::Column::Id)
    ///         .filter(condition().eq(test_db::Column::Id, 1).build())
    ///         .build(sea_orm::DatabaseBackend::Postgres)
    ///         .to_string();
    ///
    ///     assert_eq!(
    ///         query_str,
    ///         "SELECT \"loco\".\"id\" FROM \"loco\" WHERE \"loco\".\"id\" = 1"
    ///     );
    /// ````
    ///
    /// On string field
    /// ```
    /// use loco_rs::tests_cfg::db::*;
    /// use sea_orm::{EntityTrait, QueryFilter, QuerySelect, QueryTrait};
    /// use loco_rs::prelude::{model::query::*, *};
    ///
    /// let query_str = test_db::Entity::find()
    ///         .select_only()
    ///         .column(test_db::Column::Id)
    ///         .filter(condition().eq(test_db::Column::Name, "loco").build())
    ///         .build(sea_orm::DatabaseBackend::Postgres)
    ///         .to_string();
    ///
    ///     assert_eq!(
    ///         query_str,
    ///         "SELECT \"loco\".\"id\" FROM \"loco\" WHERE \"loco\".\"name\" = 'loco'"
    ///     );
    /// ````
    #[must_use]
    pub fn eq<T: ColumnTrait, V: Into<Value>>(self, col: T, value: V) -> Self {
        with(self.condition.add(col.eq(value)), self.pagination_query)
    }

    /// where condition the given column not equals the given value
    ///
    /// # Examples
    /// ```
    /// use loco_rs::tests_cfg::db::*;
    /// use sea_orm::{EntityTrait, QueryFilter, QuerySelect, QueryTrait};
    /// use loco_rs::prelude::{model::query::*, *};
    ///
    /// let query_str = test_db::Entity::find()
    ///         .select_only()
    ///         .column(test_db::Column::Id)
    ///         .filter(condition().ne(test_db::Column::Id, 1).build())
    ///         .build(sea_orm::DatabaseBackend::Postgres)
    ///         .to_string();
    ///
    ///     assert_eq!(
    ///         query_str,
    ///         "SELECT \"loco\".\"id\" FROM \"loco\" WHERE \"loco\".\"id\" <> 1"
    ///     );
    /// ````
    #[must_use]
    pub fn ne<T: ColumnTrait, V: Into<Value>>(self, col: T, value: V) -> Self {
        with(self.condition.add(col.ne(value)), self.pagination_query)
    }

    /// where condition the given column greater than the given value
    ///
    /// # Examples
    /// ```
    /// use loco_rs::tests_cfg::db::*;
    /// use sea_orm::{EntityTrait, QueryFilter, QuerySelect, QueryTrait};
    /// use loco_rs::prelude::{model::query::*, *};
    ///
    /// let query_str = test_db::Entity::find()
    ///         .select_only()
    ///         .column(test_db::Column::Id)
    ///         .filter(condition().gt(test_db::Column::Id, 1).build())
    ///         .build(sea_orm::DatabaseBackend::Postgres)
    ///         .to_string();
    ///
    ///     assert_eq!(
    ///         query_str,
    ///         "SELECT \"loco\".\"id\" FROM \"loco\" WHERE \"loco\".\"id\" > 1"
    ///     );
    /// ````
    #[must_use]
    pub fn gt<T: ColumnTrait, V: Into<Value>>(self, col: T, value: V) -> Self {
        with(self.condition.add(col.gt(value)), self.pagination_query)
    }

    /// where condition the given column greater than or equal to the given
    /// value
    ///
    /// # Examples
    /// ```
    /// use loco_rs::tests_cfg::db::*;
    /// use sea_orm::{EntityTrait, QueryFilter, QuerySelect, QueryTrait};
    /// use loco_rs::prelude::{model::query::*, *};
    ///
    /// let query_str = test_db::Entity::find()
    ///         .select_only()
    ///         .column(test_db::Column::Id)
    ///         .filter(condition().gte(test_db::Column::Id, 1).build())
    ///         .build(sea_orm::DatabaseBackend::Postgres)
    ///         .to_string();
    ///
    ///     assert_eq!(
    ///         query_str,
    ///         "SELECT \"loco\".\"id\" FROM \"loco\" WHERE \"loco\".\"id\" >= 1"
    ///     );
    /// ````
    #[must_use]
    pub fn gte<T: ColumnTrait, V: Into<Value>>(self, col: T, value: V) -> Self {
        with(self.condition.add(col.gte(value)), self.pagination_query)
    }

    /// where condition the given column smaller than to the given
    /// value
    ///
    /// # Examples
    /// ```
    /// use loco_rs::tests_cfg::db::*;
    /// use sea_orm::{EntityTrait, QueryFilter, QuerySelect, QueryTrait};
    /// use loco_rs::prelude::{model::query::*, *};
    ///
    /// let query_str = test_db::Entity::find()
    ///         .select_only()
    ///         .column(test_db::Column::Id)
    ///         .filter(condition().lt(test_db::Column::Id, 1).build())
    ///         .build(sea_orm::DatabaseBackend::Postgres)
    ///         .to_string();
    ///
    ///     assert_eq!(
    ///         query_str,
    ///         "SELECT \"loco\".\"id\" FROM \"loco\" WHERE \"loco\".\"id\" < 1"
    ///     );
    /// ````
    #[must_use]
    pub fn lt<T: ColumnTrait, V: Into<Value>>(self, col: T, value: V) -> Self {
        with(self.condition.add(col.lt(value)), self.pagination_query)
    }

    /// where condition the given column smaller than or equal to the given
    /// value
    ///
    /// # Examples
    /// ```
    /// use loco_rs::tests_cfg::db::*;
    /// use sea_orm::{EntityTrait, QueryFilter, QuerySelect, QueryTrait};
    /// use loco_rs::prelude::{model::query::*, *};
    ///
    /// let query_str = test_db::Entity::find()
    ///         .select_only()
    ///         .column(test_db::Column::Id)
    ///         .filter(condition().lte(test_db::Column::Id, 1).build())
    ///         .build(sea_orm::DatabaseBackend::Postgres)
    ///         .to_string();
    ///
    ///     assert_eq!(
    ///         query_str,
    ///         "SELECT \"loco\".\"id\" FROM \"loco\" WHERE \"loco\".\"id\" <= 1"
    ///     );
    /// ````
    #[must_use]
    pub fn lte<T: ColumnTrait, V: Into<Value>>(self, col: T, value: V) -> Self {
        with(self.condition.add(col.lte(value)), self.pagination_query)
    }

    /// where condition the given column between the given values
    /// value
    ///
    /// # Examples
    /// ```
    /// use loco_rs::tests_cfg::db::*;
    /// use sea_orm::{EntityTrait, QueryFilter, QuerySelect, QueryTrait};
    /// use loco_rs::prelude::{model::query::*, *};
    ///
    /// let query_str = test_db::Entity::find()
    ///         .select_only()
    ///         .column(test_db::Column::Id)
    ///         .filter(condition().between(test_db::Column::Id, 1, 2).build())
    ///         .build(sea_orm::DatabaseBackend::Postgres)
    ///         .to_string();
    ///
    ///     assert_eq!(
    ///         query_str,
    ///         "SELECT \"loco\".\"id\" FROM \"loco\" WHERE \"loco\".\"id\" BETWEEN 1 AND 2"
    ///     );
    /// ````
    #[must_use]
    pub fn between<T: ColumnTrait, V: Into<Value>>(self, col: T, a: V, b: V) -> Self {
        with(self.condition.add(col.between(a, b)), self.pagination_query)
    }

    /// where condition the given column not between the given values
    /// value
    ///
    /// # Examples
    /// ```
    /// use loco_rs::tests_cfg::db::*;
    /// use sea_orm::{EntityTrait, QueryFilter, QuerySelect, QueryTrait};
    /// use loco_rs::prelude::{model::query::*, *};
    ///
    /// let query_str = test_db::Entity::find()
    ///         .select_only()
    ///         .column(test_db::Column::Id)
    ///         .filter(condition().not_between(test_db::Column::Id, 1, 2).build())
    ///         .build(sea_orm::DatabaseBackend::Postgres)
    ///         .to_string();
    ///
    ///     assert_eq!(
    ///         query_str,
    ///         "SELECT \"loco\".\"id\" FROM \"loco\" WHERE \"loco\".\"id\" NOT BETWEEN 1 AND 2"
    ///     );
    /// ````
    #[must_use]
    pub fn not_between<T: ColumnTrait, V: Into<Value>>(self, col: T, a: V, b: V) -> Self {
        with(
            self.condition.add(col.not_between(a, b)),
            self.pagination_query,
        )
    }

    /// where condition the given column like given values
    /// value
    ///
    /// # Examples
    /// ```
    /// use loco_rs::tests_cfg::db::*;
    /// use sea_orm::{EntityTrait, QueryFilter, QuerySelect, QueryTrait};
    /// use loco_rs::prelude::{model::query::*, *};
    ///
    /// let query_str = test_db::Entity::find()
    ///         .select_only()
    ///         .column(test_db::Column::Id)
    ///         .filter(condition().like(test_db::Column::Name, "%lo").build())
    ///         .build(sea_orm::DatabaseBackend::Postgres)
    ///         .to_string();
    ///
    ///     assert_eq!(
    ///         query_str,
    ///         "SELECT \"loco\".\"id\" FROM \"loco\" WHERE \"loco\".\"name\" LIKE '%lo'"
    ///     );
    /// ````
    #[must_use]
    pub fn like<T: ColumnTrait, V: Into<String>>(self, col: T, a: V) -> Self {
        with(self.condition.add(col.like(a)), self.pagination_query)
    }

    /// where condition the given column not like given values
    /// value
    ///
    /// # Examples
    /// ```
    /// use loco_rs::tests_cfg::db::*;
    /// use sea_orm::{EntityTrait, QueryFilter, QuerySelect, QueryTrait};
    /// use loco_rs::prelude::{model::query::*, *};
    ///
    /// let query_str = test_db::Entity::find()
    ///         .select_only()
    ///         .column(test_db::Column::Id)
    ///         .filter(condition().not_like(test_db::Column::Name, "%lo").build())
    ///         .build(sea_orm::DatabaseBackend::Postgres)
    ///         .to_string();
    ///
    ///     assert_eq!(
    ///         query_str,
    ///         "SELECT \"loco\".\"id\" FROM \"loco\" WHERE \"loco\".\"name\" NOT LIKE '%lo'"
    ///     );
    /// ````
    #[must_use]
    pub fn not_like<T: ColumnTrait, V: Into<String>>(self, col: T, a: V) -> Self {
        with(self.condition.add(col.not_like(a)), self.pagination_query)
    }

    /// where condition the given column start with given values
    /// value
    ///
    /// # Examples
    /// ```
    /// use loco_rs::tests_cfg::db::*;
    /// use sea_orm::{EntityTrait, QueryFilter, QuerySelect, QueryTrait};
    /// use loco_rs::prelude::{model::query::*, *};
    ///
    /// let query_str = test_db::Entity::find()
    ///         .select_only()
    ///         .column(test_db::Column::Id)
    ///         .filter(condition().starts_with(test_db::Column::Name, "lo").build())
    ///         .build(sea_orm::DatabaseBackend::Postgres)
    ///         .to_string();
    ///
    ///     assert_eq!(
    ///         query_str,
    ///         "SELECT \"loco\".\"id\" FROM \"loco\" WHERE \"loco\".\"name\" LIKE 'lo%'"
    ///     );
    /// ````
    #[must_use]
    pub fn starts_with<T: ColumnTrait, V: Into<String>>(self, col: T, a: V) -> Self {
        with(
            self.condition.add(col.starts_with(a)),
            self.pagination_query,
        )
    }

    /// where condition the given column end with given values
    /// value
    ///
    /// # Examples
    /// ```
    /// use loco_rs::tests_cfg::db::*;
    /// use sea_orm::{EntityTrait, QueryFilter, QuerySelect, QueryTrait};
    /// use loco_rs::prelude::{model::query::*, *};
    ///
    /// let query_str = test_db::Entity::find()
    ///         .select_only()
    ///         .column(test_db::Column::Id)
    ///         .filter(condition().ends_with(test_db::Column::Name, "lo").build())
    ///         .build(sea_orm::DatabaseBackend::Postgres)
    ///         .to_string();
    ///
    ///     assert_eq!(
    ///         query_str,
    ///         "SELECT \"loco\".\"id\" FROM \"loco\" WHERE \"loco\".\"name\" LIKE '%lo'"
    ///     );
    /// ````
    #[must_use]
    pub fn ends_with<T: ColumnTrait, V: Into<String>>(self, col: T, a: V) -> Self {
        with(self.condition.add(col.ends_with(a)), self.pagination_query)
    }

    /// where condition the given column end with given values
    /// value
    ///
    /// # Examples
    /// ```
    /// use loco_rs::tests_cfg::db::*;
    /// use sea_orm::{EntityTrait, QueryFilter, QuerySelect, QueryTrait};
    /// use loco_rs::prelude::{model::query::*, *};
    ///
    /// let query_str = test_db::Entity::find()
    ///         .select_only()
    ///         .column(test_db::Column::Id)
    ///         .filter(condition().contains(test_db::Column::Name, "lo").build())
    ///         .build(sea_orm::DatabaseBackend::Postgres)
    ///         .to_string();
    ///
    ///     assert_eq!(
    ///         query_str,
    ///         "SELECT \"loco\".\"id\" FROM \"loco\" WHERE \"loco\".\"name\" LIKE '%lo%'"
    ///     );
    /// ````
    #[must_use]
    pub fn contains<T: ColumnTrait, V: Into<String>>(self, col: T, a: V) -> Self {
        with(self.condition.add(col.contains(a)), self.pagination_query)
    }

    /// where condition the given column is null
    /// value
    ///
    /// # Examples
    /// ```
    /// use loco_rs::tests_cfg::db::*;
    /// use sea_orm::{EntityTrait, QueryFilter, QuerySelect, QueryTrait};
    /// use loco_rs::prelude::{model::query::*, *};
    ///
    /// let query_str = test_db::Entity::find()
    ///         .select_only()
    ///         .column(test_db::Column::Id)
    ///         .filter(condition().is_null(test_db::Column::Name).build())
    ///         .build(sea_orm::DatabaseBackend::Postgres)
    ///         .to_string();
    ///
    ///     assert_eq!(
    ///         query_str,
    ///         "SELECT \"loco\".\"id\" FROM \"loco\" WHERE \"loco\".\"name\" IS NULL"
    ///     );
    /// ````
    #[must_use]
    #[allow(clippy::wrong_self_convention)]
    pub fn is_null<T: ColumnTrait>(self, col: T) -> Self {
        with(self.condition.add(col.is_null()), self.pagination_query)
    }

    /// where condition the given column is not null
    /// value
    ///
    /// # Examples
    /// ```
    /// use loco_rs::tests_cfg::db::*;
    /// use sea_orm::{EntityTrait, QueryFilter, QuerySelect, QueryTrait};
    /// use loco_rs::prelude::{model::query::*, *};
    ///
    /// let query_str = test_db::Entity::find()
    ///         .select_only()
    ///         .column(test_db::Column::Id)
    ///         .filter(condition().is_not_null(test_db::Column::Name).build())
    ///         .build(sea_orm::DatabaseBackend::Postgres)
    ///         .to_string();
    ///
    ///     assert_eq!(
    ///         query_str,
    ///         "SELECT \"loco\".\"id\" FROM \"loco\" WHERE \"loco\".\"name\" IS NOT NULL"
    ///     );
    /// ````
    #[must_use]
    #[allow(clippy::wrong_self_convention)]
    pub fn is_not_null<T: ColumnTrait>(self, col: T) -> Self {
        with(self.condition.add(col.is_not_null()), self.pagination_query)
    }

    /// where condition the given column is not null
    /// value
    ///
    /// # Examples
    /// ```
    /// use loco_rs::tests_cfg::db::*;
    /// use sea_orm::{EntityTrait, QueryFilter, QuerySelect, QueryTrait};
    /// use loco_rs::prelude::{model::query::*, *};
    ///
    /// let from_date = chrono::NaiveDateTime::parse_from_str("2024-03-01
    /// 22:10:57", "%Y-%m-%d %H:%M:%S").unwrap(); let to_date =
    /// chrono::NaiveDateTime::parse_from_str("2024-03-25 22:10:57", "%Y-%m-%d
    /// %H:%M:%S").unwrap();
    ///
    /// let condition = condition()
    ///     .date_range(test_db::Column::CreatedAt)
    ///     .dates(Some(&from_date), Some(&to_date))
    ///     .build();
    ///
    /// let query_str = test_db::Entity::find()
    ///     .select_only()
    ///     .column(test_db::Column::Id)
    ///     .filter(condition.build())
    ///     .build(sea_orm::DatabaseBackend::Postgres)
    ///     .to_string();
    ///
    /// assert_eq!(
    ///     query_str,
    ///     "SELECT \"loco\".\"id\" FROM \"loco\" WHERE \"loco\".\"created_at\" BETWEEN '2024-03-01 22:10:57' AND '2024-03-25 22:10:57'" );
    /// ````
    #[must_use]
    pub fn date_range<T: ColumnTrait>(self, col: T) -> date_range::DateRangeBuilder<T> {
        date_range::DateRangeBuilder::new(self, col)
    }

    /// Adding page size for paging query. default is [`pagination::default_page_size`]
    ///
    /// # Examples
    ///
    /// ```
    /// use loco_rs::tests_cfg::db::*;
    /// use sea_orm::{EntityTrait, QueryFilter, QuerySelect, QueryTrait};
    /// use loco_rs::prelude::{model::query::*, *};
    ///
    /// async fn example() {
    ///     let db = dummy_connection().await;
    ///     let res = condition()
    ///         .page_size(100)
    ///         .paginate::<test_db::Entity>(&db).await;
    /// }
    /// ````
    #[must_use]
    pub fn page_size(self, size: u64) -> Self {
        let mut pagination_query = self.pagination_query;
        pagination_query.page_size = size;
        with(self.condition, pagination_query)
    }

    /// Adding page for paging query. default is [`pagination::default_page`]
    ///
    /// # Examples
    ///
    /// ```
    /// use loco_rs::tests_cfg::db::*;
    /// use sea_orm::{EntityTrait, QueryFilter, QuerySelect, QueryTrait};
    /// use loco_rs::prelude::{model::query::*, *};
    ///
    /// async fn example() {
    ///     let db = dummy_connection().await;
    ///     let res = condition()
    ///         .page(2)
    ///         .paginate::<test_db::Entity>(&db).await;
    /// }
    /// ````
    #[must_use]
    pub fn page(self, page: u64) -> Self {
        let mut pagination_query = self.pagination_query;
        pagination_query.page = page;
        with(self.condition, pagination_query)
    }

    /// Setting pagination query struct
    ///
    /// # Examples
    ///
    /// ```
    /// use loco_rs::tests_cfg::db::*;
    /// use sea_orm::{EntityTrait, QueryFilter, QuerySelect, QueryTrait};
    /// use loco_rs::prelude::{model::query::*, *};
    ///
    /// async fn example() {
    ///     let db = dummy_connection().await;
    ///     let pagination_query = pagination::PaginationQuery {
    ///         page_size: 100,
    ///         page: 1,
    ///     };
    ///     let res = condition()
    ///         .pagination_query(pagination_query)
    ///         .paginate::<test_db::Entity>(&db).await;
    /// }
    /// ````
    #[must_use]
    pub fn pagination_query(self, pagination_query: pagination::PaginationQuery) -> Self {
        with(self.condition, pagination_query)
    }

    /// Execute the pagination query with the builder arguments
    ///
    /// # Errors
    /// When could not execute the query
    ///
    /// # Examples
    ///
    /// ```
    /// use loco_rs::tests_cfg::db::*;
    /// use sea_orm::{EntityTrait, QueryFilter, QuerySelect, QueryTrait};
    /// use loco_rs::prelude::{model::query::*, *};
    ///
    /// async fn example() {
    ///     let db = dummy_connection().await;
    ///     let res = condition()
    ///         .contains(test_db::Column::Name, "Loco")
    ///         .page_size(100)
    ///         .paginate::<test_db::Entity>(&db).await;
    /// }
    /// ````
    pub async fn paginate<E>(
        &self,
        db: &DatabaseConnection,
    ) -> LocoResult<pagination::PaginatedResponse<E::Model>>
    where
        E: EntityTrait,
        <E as EntityTrait>::Model: Sync,
    {
        let filters = self.build();

        let notes_entity = E::find();
        let paginated_response =
            pagination::paginate::<E>(db, notes_entity, Some(filters), &self.pagination_query)
                .await?;

        Ok(paginated_response)
    }

    /// Execute the pagination query with sorting
    ///
    /// # Errors
    /// When could not execute the query
    ///
    /// # Examples
    ///
    /// ```
    /// use loco_rs::tests_cfg::db::*;
    /// use sea_orm::{EntityTrait, QueryFilter, QuerySelect, QueryTrait};
    /// use loco_rs::prelude::{model::query::*, *};
    ///
    /// async fn example() {
    ///     let db = dummy_connection().await;
    ///     let res = condition()
    ///         .contains(test_db::Column::Name, "Loco")
    ///         .page_size(100)
    ///         .paginate_with_order::<test_db::Entity, test_db::Column>(&db, test_db::Column::Name, SortDirection::Desc).await;
    /// }
    /// ````
    pub async fn paginate_with_order<E, C>(
        &self,
        db: &DatabaseConnection,
        col: C,
        direction: SortDirection,
    ) -> LocoResult<pagination::PaginatedResponse<E::Model>>
    where
        E: EntityTrait,
        <E as EntityTrait>::Model: Sync,
        C: ColumnTrait,
    {
        let filters = self.build();

        let notes_entity = E::find().order_by(col, direction.order());
        let paginated_response =
            pagination::paginate::<E>(db, notes_entity, Some(filters), &self.pagination_query)
                .await?;

        Ok(paginated_response)
    }

    #[must_use]
    pub fn build(&self) -> Condition {
        self.condition.clone().into_condition()
    }
}

#[cfg(test)]
mod tests {

    use sea_orm::{EntityTrait, QueryFilter, QuerySelect, QueryTrait};

    use super::*;
    use crate::tests_cfg::db::*;

    #[test]
    fn condition_eq() {
        let query_str = test_db::Entity::find()
            .select_only()
            .column(test_db::Column::Id)
            .filter(condition().eq(test_db::Column::Id, 1).build())
            .build(sea_orm::DatabaseBackend::Postgres)
            .to_string();

        assert_eq!(
            query_str,
            "SELECT \"loco\".\"id\" FROM \"loco\" WHERE \"loco\".\"id\" = 1"
        );
    }

    #[test]
    fn condition_ne() {
        let query_str = test_db::Entity::find()
            .select_only()
            .column(test_db::Column::Id)
            .filter(condition().ne(test_db::Column::Name, "loco").build())
            .build(sea_orm::DatabaseBackend::Postgres)
            .to_string();

        assert_eq!(
            query_str,
            "SELECT \"loco\".\"id\" FROM \"loco\" WHERE \"loco\".\"name\" <> 'loco'"
        );
    }

    #[test]
    fn condition_gt() {
        let query_str = test_db::Entity::find()
            .select_only()
            .column(test_db::Column::Id)
            .filter(condition().gt(test_db::Column::Id, 1).build())
            .build(sea_orm::DatabaseBackend::Postgres)
            .to_string();

        assert_eq!(
            query_str,
            "SELECT \"loco\".\"id\" FROM \"loco\" WHERE \"loco\".\"id\" > 1"
        );
    }

    #[test]
    fn condition_gte() {
        let query_str = test_db::Entity::find()
            .select_only()
            .column(test_db::Column::Id)
            .filter(condition().gte(test_db::Column::Id, 1).build())
            .build(sea_orm::DatabaseBackend::Postgres)
            .to_string();

        assert_eq!(
            query_str,
            "SELECT \"loco\".\"id\" FROM \"loco\" WHERE \"loco\".\"id\" >= 1"
        );
    }

    #[test]
    fn condition_lt() {
        let query_str = test_db::Entity::find()
            .select_only()
            .column(test_db::Column::Id)
            .filter(condition().lt(test_db::Column::Id, 1).build())
            .build(sea_orm::DatabaseBackend::Postgres)
            .to_string();

        assert_eq!(
            query_str,
            "SELECT \"loco\".\"id\" FROM \"loco\" WHERE \"loco\".\"id\" < 1"
        );
    }

    #[test]
    fn condition_lte() {
        let query_str = test_db::Entity::find()
            .select_only()
            .column(test_db::Column::Id)
            .filter(condition().lte(test_db::Column::Id, 1).build())
            .build(sea_orm::DatabaseBackend::Postgres)
            .to_string();

        assert_eq!(
            query_str,
            "SELECT \"loco\".\"id\" FROM \"loco\" WHERE \"loco\".\"id\" <= 1"
        );
    }

    #[test]
    fn condition_between() {
        let query_str = test_db::Entity::find()
            .select_only()
            .column(test_db::Column::Id)
            .filter(condition().between(test_db::Column::Id, 1, 2).build())
            .build(sea_orm::DatabaseBackend::Postgres)
            .to_string();

        assert_eq!(
            query_str,
            "SELECT \"loco\".\"id\" FROM \"loco\" WHERE \"loco\".\"id\" BETWEEN 1 AND 2"
        );
    }

    #[test]
    fn condition_not_between() {
        let query_str = test_db::Entity::find()
            .select_only()
            .column(test_db::Column::Id)
            .filter(condition().not_between(test_db::Column::Id, 1, 2).build())
            .build(sea_orm::DatabaseBackend::Postgres)
            .to_string();

        assert_eq!(
            query_str,
            "SELECT \"loco\".\"id\" FROM \"loco\" WHERE \"loco\".\"id\" NOT BETWEEN 1 AND 2"
        );
    }

    #[test]
    fn condition_like() {
        let query_str = test_db::Entity::find()
            .select_only()
            .column(test_db::Column::Id)
            .filter(condition().like(test_db::Column::Name, "%lo").build())
            .build(sea_orm::DatabaseBackend::Postgres)
            .to_string();

        assert_eq!(
            query_str,
            "SELECT \"loco\".\"id\" FROM \"loco\" WHERE \"loco\".\"name\" LIKE '%lo'"
        );
    }

    #[test]
    fn condition_not_like() {
        let query_str = test_db::Entity::find()
            .select_only()
            .column(test_db::Column::Id)
            .filter(condition().not_like(test_db::Column::Name, "%lo%").build())
            .build(sea_orm::DatabaseBackend::Postgres)
            .to_string();

        assert_eq!(
            query_str,
            "SELECT \"loco\".\"id\" FROM \"loco\" WHERE \"loco\".\"name\" NOT LIKE '%lo%'"
        );
    }

    #[test]
    fn condition_starts_with() {
        let query_str = test_db::Entity::find()
            .select_only()
            .column(test_db::Column::Id)
            .filter(condition().starts_with(test_db::Column::Name, "lo").build())
            .build(sea_orm::DatabaseBackend::Postgres)
            .to_string();

        assert_eq!(
            query_str,
            "SELECT \"loco\".\"id\" FROM \"loco\" WHERE \"loco\".\"name\" LIKE 'lo%'"
        );
    }

    #[test]
    fn condition_ends_with() {
        let query_str = test_db::Entity::find()
            .select_only()
            .column(test_db::Column::Id)
            .filter(condition().ends_with(test_db::Column::Name, "lo").build())
            .build(sea_orm::DatabaseBackend::Postgres)
            .to_string();

        assert_eq!(
            query_str,
            "SELECT \"loco\".\"id\" FROM \"loco\" WHERE \"loco\".\"name\" LIKE '%lo'"
        );
    }

    #[test]
    fn condition_contains() {
        let query_str = test_db::Entity::find()
            .select_only()
            .column(test_db::Column::Id)
            .filter(condition().contains(test_db::Column::Name, "lo").build())
            .build(sea_orm::DatabaseBackend::Postgres)
            .to_string();

        assert_eq!(
            query_str,
            "SELECT \"loco\".\"id\" FROM \"loco\" WHERE \"loco\".\"name\" LIKE '%lo%'"
        );
    }

    #[test]
    fn condition_is_null() {
        let query_str = test_db::Entity::find()
            .select_only()
            .column(test_db::Column::Id)
            .filter(condition().is_null(test_db::Column::Name).build())
            .build(sea_orm::DatabaseBackend::Postgres)
            .to_string();

        assert_eq!(
            query_str,
            "SELECT \"loco\".\"id\" FROM \"loco\" WHERE \"loco\".\"name\" IS NULL"
        );
    }

    #[test]
    fn condition_is_not_null() {
        let query_str = test_db::Entity::find()
            .select_only()
            .column(test_db::Column::Id)
            .filter(condition().is_not_null(test_db::Column::Name).build())
            .build(sea_orm::DatabaseBackend::Postgres)
            .to_string();

        assert_eq!(
            query_str,
            "SELECT \"loco\".\"id\" FROM \"loco\" WHERE \"loco\".\"name\" IS NOT NULL"
        );
    }
}