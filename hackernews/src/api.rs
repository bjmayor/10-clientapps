#![allow(unused)]
use crate::{Comment, StoryData, StoryItem};
use anyhow::Result;
use futures::future::join_all;
const MAX_STORIES: usize = 50;
pub async fn get_top_stories(n: usize) -> Result<Vec<StoryItem>, Box<dyn std::error::Error>> {
    let n = n.min(MAX_STORIES);
    let url = "https://hacker-news.firebaseio.com/v0/topstories.json";
    let story_ids: Vec<i64> = reqwest::get(url).await?.json().await?;
    let story_futures = story_ids.iter().take(n).map(|id| get_story_item_by_id(*id));
    let stories = join_all(story_futures)
        .await
        .into_iter()
        .filter_map(|x| x.ok())
        .collect::<Vec<StoryItem>>();
    Ok(stories)
}

pub async fn get_story_item_by_id(id: i64) -> Result<StoryItem> {
    let url = format!("https://hacker-news.firebaseio.com/v0/item/{}.json", id);
    let story: StoryItem = reqwest::get(&url).await?.json().await?;
    Ok(story)
}

pub async fn get_comment_by_id(id: i64) -> Result<Comment> {
    let url = format!("https://hacker-news.firebaseio.com/v0/item/{}.json", id);
    let comment: Comment = reqwest::get(&url).await?.json().await?;
    Ok(comment)
}

pub async fn get_story_comments(item: StoryItem) -> Result<StoryData> {
    let comment_futures = item.kids.iter().map(|id| get_comment_by_id(*id));
    let comments = join_all(comment_futures)
        .await
        .into_iter()
        .filter_map(|x| x.ok())
        .collect::<Vec<Comment>>();
    Ok(StoryData {
        item,
        comments: comments.into_iter().collect(),
    })
}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn get_top_stories_should_work() {
        let stories = get_top_stories(3).await.unwrap();
        assert_eq!(stories.len(), 3);
    }

    #[tokio::test]
    async fn get_comment_by_id_should_work() {
        let story = get_top_stories(1).await.unwrap().pop().unwrap();
        let id = story.kids.first().unwrap();
        let comment = get_comment_by_id(*id).await.unwrap();
        assert_eq!(comment.id, *id);
    }
}
