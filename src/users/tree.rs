use atree::Arena;
use axum::{http::StatusCode, routing::get, Router};
use serde::{Deserialize, Serialize};

pub fn create_routes() -> Router {
    Router::new().route("/tree-structure", get(test_tree_struct))
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Comment {
    pub comment_id: i32,
    pub parent_com_id: Option<i32>,
    pub title: String,
}

fn fake_comments() -> Vec<Comment> {
    let comment1 = Comment {
        comment_id: 1,
        parent_com_id: None,
        title: "What is the best Linux distribution".to_string(),
    };
    let comment2 = Comment {
        comment_id: 2,
        parent_com_id: None,
        title: "How to create React project".to_string(),
    };

    let comment3 = Comment {
        comment_id: 3,
        parent_com_id: Some(2),
        title: "use `npx run create-react-app`".to_string(),
    };

    let comment4 = Comment {
        comment_id: 4,
        parent_com_id: Some(3),
        title: "you can use `yarn run create-react-app` too".to_string(),
    };
    let comment5 = Comment {
        comment_id: 5,
        parent_com_id: Some(2),
        title: "vite command also can create project".to_string(),
    };
    let comment6 = Comment {
        comment_id: 6,
        parent_com_id: Some(3),
        title: "reply to comment id [2]".to_string(),
    };
    vec![comment6, comment1, comment5, comment4, comment3, comment2]
}

fn _get_children(parent_id: &i32) -> Vec<Comment> {
    let children = get_children(parent_id).unwrap();
    let mut descendant: Vec<Comment> = vec![];
    children.iter().for_each(|child| {
        // TODO: got all descendant of this child
        tracing::debug!("{:#?}", &child);
        descendant.push(child.clone());
        _get_children(&child.comment_id)
            .iter()
            .for_each(|child| descendant.push(child.clone()));
    });
    descendant
}

async fn test_tree_struct() -> Result<String, StatusCode> {
    let root = get_root(2).unwrap();
    let (mut arena, root_token) = Arena::with_data(&root);
    let children = _get_children(&root.comment_id);
    children.iter().for_each(|child| {
        root_token.append(&mut arena, child);
    });

    Ok(format!(
        "{:#?}",
        root_token
            .children(&arena)
            .map(|child| child.data.clone())
            .collect::<Vec<Comment>>()
    ))
}

fn get_root(root_id: i32) -> Option<Comment> {
    let comments = fake_comments();
    for comm in comments {
        if comm.comment_id == root_id {
            return Some(comm);
        }
    }
    None
}

fn get_children(parent_id: &i32) -> Option<Vec<Comment>> {
    let comments = fake_comments();
    // let mut children: Vec<Comment> = vec![];
    // for comm in comments {
    //     if let Some(parent_com_id) = comm.parent_com_id {
    //         if parent_com_id == *parent_id {
    //             children.push(comm)
    //         }
    //     }
    // }
    let children: Vec<Comment> = comments
        .iter()
        .filter(|comment| {
            comment.parent_com_id.is_some() && comment.parent_com_id.unwrap() == *parent_id
        })
        .map(|c| c.clone())
        .collect();

    Some(children)
}
