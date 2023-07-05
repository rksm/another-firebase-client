use super::conversion::IntoFirestoreDocumentValue;
use firestore_grpc::v1::{self as firestore, structured_query::CollectionSelector};

pub use prost_types::Timestamp;

pub type UnaryFilterOperator = firestore::structured_query::unary_filter::Operator;
pub type FieldFilterOperator = firestore::structured_query::field_filter::Operator;
pub type CompositeFilterOperator = firestore::structured_query::composite_filter::Operator;
pub type Filter = firestore::structured_query::Filter;

#[derive(Clone)]
pub struct StructuredQueryBuilder {
    pub order_by: Vec<firestore::structured_query::Order>,
    pub filter: Option<Filter>,
    pub limit: Option<i32>,
    pub offset: i32,
    pub start_at: Option<firestore::Cursor>,
    pub end_at: Option<firestore::Cursor>,
    pub from_collections: Vec<String>,
}

impl StructuredQueryBuilder {
    pub fn new() -> Self {
        Self {
            limit: None,
            offset: 0,
            start_at: None,
            end_at: None,
            order_by: Vec::new(),
            filter: None,
            from_collections: Vec::new(),
        }
    }

    pub fn from<S: ToString>(&mut self, collection_id: S) {
        self.from_collections.push(collection_id.to_string())
    }

    pub fn limit(&mut self, limit: i32) {
        self.limit = Some(limit);
    }

    pub fn offset(&mut self, offset: i32) {
        self.offset = offset;
    }

    pub fn order_by<S: ToString>(self, field: S) -> OrderBuilder<Self> {
        OrderBuilder::new(
            self,
            |me, order, start_at, end_at| {
                me.order_by.push(order);
                me.start_at = start_at;
                me.end_at = end_at;
            },
            field,
        )
    }

    pub fn unary_filter<S: ToString>(&mut self, field: S, op: UnaryFilterOperator) {
        self.filter = Some(unary_filter(field, op));
    }

    pub fn field_filter<T, S>(&mut self, field: S, op: FieldFilterOperator, value: T)
    where
        T: IntoFirestoreDocumentValue,
        S: ToString,
    {
        self.filter = Some(field_filter(field, op, value));
    }

    pub fn composite_filter(&mut self, op: CompositeFilterOperator, filters: Vec<Filter>) {
        self.filter = Some(composite_filter(op, filters));
    }

    pub fn build(self) -> firestore::StructuredQuery {
        let Self {
            order_by,
            filter,
            limit,
            offset,
            start_at,
            end_at,
            from_collections,
        } = self;
        firestore::StructuredQuery {
            select: None,
            from: from_collections
                .into_iter()
                .map(|c| CollectionSelector {
                    collection_id: c,
                    all_descendants: false,
                })
                .collect(),
            r#where: filter,
            order_by,
            start_at,
            end_at,
            offset,
            limit,
        }
    }
}

pub fn unary_filter<S: ToString>(field: S, op: UnaryFilterOperator) -> Filter {
    use firestore::structured_query::*;

    let field = FieldReference {
        field_path: field.to_string(),
    };

    let operand_type = unary_filter::OperandType::Field(field);
    Filter {
        filter_type: Some(filter::FilterType::UnaryFilter(UnaryFilter {
            op: op.into(),
            operand_type: Some(operand_type),
        })),
    }
}

pub fn field_filter<T, S>(field: S, op: FieldFilterOperator, value: T) -> Filter
where
    T: IntoFirestoreDocumentValue,
    S: ToString,
{
    use firestore::structured_query::*;

    let field = FieldReference {
        field_path: field.to_string(),
    };
    let value = value.into_document_value();

    Filter {
        filter_type: Some(filter::FilterType::FieldFilter(FieldFilter {
            op: op.into(),
            field: Some(field),
            value: Some(value),
        })),
    }
}

pub fn composite_filter(op: CompositeFilterOperator, filters: Vec<Filter>) -> Filter {
    use firestore::structured_query::*;
    Filter {
        filter_type: Some(filter::FilterType::CompositeFilter(CompositeFilter {
            op: op.into(),
            filters,
        })),
    }
}

type OrderInstaller<T> = fn(
    &mut T,
    order: firestore::structured_query::Order,
    start_at: Option<firestore::Cursor>,
    end_at: Option<firestore::Cursor>,
);

pub struct OrderBuilder<T> {
    pub install_target: T,
    pub install: OrderInstaller<T>,
    pub order: firestore::structured_query::Order,
    pub start_at: Option<firestore::Cursor>,
    pub end_at: Option<firestore::Cursor>,
}

impl<T> OrderBuilder<T> {
    pub fn new<S: ToString>(install_target: T, install: OrderInstaller<T>, field: S) -> Self {
        Self {
            install_target,
            install,
            order: firestore::structured_query::Order {
                field: Some(firestore::structured_query::FieldReference {
                    field_path: field.to_string(),
                }),
                ..Default::default()
            },
            start_at: None,
            end_at: None,
        }
    }

    pub fn descending(mut self) -> Self {
        self.order.direction = firestore::structured_query::Direction::Descending.into();
        self
    }

    pub fn ascending(mut self) -> Self {
        self.order.direction = firestore::structured_query::Direction::Ascending.into();
        self
    }

    fn add_cursor_value(
        &self,
        cursor: Option<firestore::Cursor>,
        value: impl IntoFirestoreDocumentValue,
        before: bool,
    ) -> Option<firestore::Cursor> {
        let mut cursor = cursor.unwrap_or(firestore::Cursor {
            values: Vec::new(),
            before: false,
        });
        cursor.values.push(value.into_document_value());
        cursor.before = before;
        Some(cursor)
    }

    pub fn end_at(mut self, value: impl IntoFirestoreDocumentValue) -> Self {
        let cursor = self.end_at.take();
        self.end_at = self.add_cursor_value(cursor, value, false);
        self
    }

    pub fn end_at_before(mut self, value: impl IntoFirestoreDocumentValue) -> Self {
        let cursor = self.end_at.take();
        self.end_at = self.add_cursor_value(cursor, value, true);
        self
    }

    pub fn start_at(mut self, value: impl IntoFirestoreDocumentValue) -> Self {
        let cursor = self.start_at.take();
        self.start_at = self.add_cursor_value(cursor, value, false);
        self
    }

    pub fn start_at_before(mut self, value: impl IntoFirestoreDocumentValue) -> Self {
        let cursor = self.start_at.take();
        self.start_at = self.add_cursor_value(cursor, value, true);
        self
    }

    pub fn done(self) -> T {
        let Self {
            mut install_target,
            install,
            order,
            start_at,
            end_at,
        } = self;
        install(&mut install_target, order, start_at, end_at);
        install_target
    }
}
