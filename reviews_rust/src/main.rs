use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Schema, SimpleObject};
use async_graphql_warp::graphql;
use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::{Arc, RwLock};
use warp::{Filter, Reply};

type SchemaDefinitionHolder = Arc<RwLock<HashMap<String, SchemaDefinition>>>;

struct Query;

#[Object]
impl Query {
    async fn reviews(&self, upc: i32) -> Vec<Review> {
        get_reviews().into_iter().filter(|r| r.upc == upc).collect()
    }

    async fn reviews_by_author(&self, author_id: i32) -> Vec<Review> {
        get_reviews()
            .into_iter()
            .filter(|r| r.author_id == author_id)
            .collect()
    }

    async fn reviews_by_product(&self, upc: i32) -> Vec<Review> {
        get_reviews().into_iter().filter(|r| r.upc == upc).collect()
    }

    #[graphql(name = "_schemaDefinition")]
    async fn _schema_definition(
        &self,
        ctx: &Context<'_>,
        configuration: String,
    ) -> Option<SchemaDefinition> {
        let result = ctx.data::<SchemaDefinitionHolder>().unwrap();
        let guard = result
            .read().unwrap();

        let schema_definition = guard.get(&configuration);

        schema_definition.map(|s| s.clone())
    }
}

#[derive(SimpleObject)]
struct Review {
    id: i32,
    author_id: i32,
    upc: i32,
    body: String,
}

#[derive(SimpleObject, Clone)]
struct SchemaDefinition {
    pub name: String,
    pub document: String,
    pub extension_documents: Vec<String>,
}

#[tokio::main]
async fn main() {
    let schema_holder = Arc::new(RwLock::new(HashMap::new()));
    let schema = Schema::build(Query, EmptyMutation, EmptySubscription)
        .data(schema_holder.clone())
        .finish();

    let schema_def = SchemaDefinition {
        name: "reviews".to_string(),
        document: schema.sdl(),
        extension_documents: vec![
            std::include_str!("Stitching.graphql").to_string(),
            "extend schema @_removeRootTypes {  }".to_string(),
        ],
    };

    let mut schema_holder_guard = schema_holder.write().unwrap();
    schema_holder_guard.insert(schema_def.name.clone(), schema_def);
    drop(schema_holder_guard);

    let graphql_filter = warp::path("graphql").and(graphql(schema).and_then(
        |(schema, request): (
            Schema<Query, EmptyMutation, EmptySubscription>,
            async_graphql::Request,
        )| async move {
            Ok::<_, Infallible>(warp::reply::json(&schema.execute(request).await).into_response())
        },
    ));
    warp::serve(graphql_filter).run(([0, 0, 0, 0], 5054)).await;
}

fn get_reviews() -> Vec<Review> {
    vec![
        Review {
            author_id: 1,
            body: "Love it!".to_string(),
            id: 1,
            upc: 1,
        },
        Review {
            author_id: 1,
            body: "Too expensive.".to_string(),
            id: 2,
            upc: 2,
        },
        Review {
            author_id: 2,
            body: "Could be better.".to_string(),
            id: 3,
            upc: 3,
        },
        Review {
            author_id: 2,
            body: "Prefer something else.".to_string(),
            id: 4,
            upc: 1,
        },
    ]
}
