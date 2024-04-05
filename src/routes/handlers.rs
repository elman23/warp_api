use super::models::Post;
use warp::Filter;

pub async fn get_post(id: u64) -> Result<impl warp::Reply, warp::Rejection> {
    let post = Post {
        id,
        title: String::from("Hello, Warp!"),
        body: String::from("This is a post about Warp."),
    };
    Ok(warp::reply::json(&post))
}
