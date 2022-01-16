use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct AggregateRoot<ID> {
    id: ID,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
}

impl<ID> AggregateRoot<ID> {
    pub fn new(id: ID) -> AggregateRoot<ID> {
        AggregateRoot {
            id,
            created_at: Utc::now(),
            updated_at: None,
        }
    }

    pub fn build(
        id: ID,
        created_at: DateTime<Utc>,
        updated_at: Option<DateTime<Utc>>,
    ) -> Self {
        AggregateRoot {
            id,
            created_at,
            updated_at,
        }
    }

    pub fn id(&self) -> &ID {
        &self.id
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    pub fn updated_at(&self) -> Option<&DateTime<Utc>> {
        self.updated_at.as_ref()
    }

    pub fn update(&mut self) {
        self.updated_at = Some(Utc::now());
    }
}

impl<ID: PartialEq> PartialEq for AggregateRoot<ID> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<ID: Clone> Clone for AggregateRoot<ID> {
    fn clone(&self) -> Self {
        AggregateRoot {
            id: self.id.clone(),
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}
