use async_graphql::Object;
use std::borrow::Cow;

use crate::S3;

#[Object]
impl S3 {
    pub async fn access_key_id(&self) -> Option<Cow<String>> {
        if !self.access_key_id.is_empty() {
            return Some(Cow::Borrowed(&self.access_key_id));
        }

        None
    }

    pub async fn secret_access_key(&self) -> Option<Cow<String>> {
        if !self.secret_access_key.is_empty() {
            return Some(Cow::Borrowed(&self.secret_access_key));
        }

        None
    }

    pub async fn bucket(&self) -> Option<Cow<String>> {
        if !self.bucket.is_empty() {
            return Some(Cow::Borrowed(&self.bucket));
        }

        None
    }

    pub async fn path(&self) -> Option<Cow<String>> {
        if !self.path.is_empty() {
            return Some(Cow::Borrowed(&self.path));
        }

        None
    }

    pub async fn region(&self) -> Option<Cow<String>> {
        if !self.region.is_empty() {
            return Some(Cow::Borrowed(&self.region));
        }

        None
    }

    pub async fn image_thumbnail_small_size(&self) -> i32 {
        self.image_thumbnail_small_size
    }

    pub async fn image_thumbnail_medium_size(&self) -> i32 {
        self.image_thumbnail_medium_size
    }

    pub async fn image_thumbnail_large_size(&self) -> i32 {
        self.image_thumbnail_large_size
    }

    pub async fn image_thumbnail_xl_size(&self) -> i32 {
        self.image_thumbnail_xl_size
    }

    pub async fn image_landscape_width_small_size(&self) -> i32 {
        self.image_landscape_width_small_size
    }

    pub async fn image_landscape_height_small_size(&self) -> i32 {
        self.image_landscape_height_small_size
    }

    pub async fn image_landscape_width_medium_size(&self) -> i32 {
        self.image_landscape_width_medium_size
    }

    pub async fn image_landscape_height_medium_size(&self) -> i32 {
        self.image_landscape_height_medium_size
    }

    pub async fn image_landscape_width_large_size(&self) -> i32 {
        self.image_landscape_width_large_size
    }

    pub async fn image_landscape_height_large_size(&self) -> i32 {
        self.image_landscape_height_large_size
    }

    pub async fn image_landscape_width_xl_size(&self) -> i32 {
        self.image_landscape_width_xl_size
    }

    pub async fn image_landscape_height_xl_size(&self) -> i32 {
        self.image_landscape_height_xl_size
    }

    pub async fn image_landscape_width_xxl_size(&self) -> i32 {
        self.image_landscape_width_xxl_size
    }

    pub async fn image_landscape_height_xxl_size(&self) -> i32 {
        self.image_landscape_height_xxl_size
    }

    pub async fn image_landscape_width_xxxl_size(&self) -> i32 {
        self.image_landscape_width_xxxl_size
    }

    pub async fn image_landscape_height_xxxl_size(&self) -> i32 {
        self.image_landscape_height_xxxl_size
    }
}