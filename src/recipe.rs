use serde::{Serialize, Deserialize};

use crate::md::{Md};
pub type Recipes = Vec<Recipe>;

#[derive(Serialize, Deserialize)]
pub struct Recipe {
    #[serde(rename = "total_time")]
    total_time: Option<String>,

    #[serde(rename = "photos")]
    photos: Vec<Option<serde_json::Value>>,

    #[serde(rename = "image_url")]
    image_url: Option<String>,

    #[serde(rename = "nutritional_info")]
    nutritional_info: Option<String>,

    #[serde(rename = "description")]
    description: Option<String>,

    #[serde(rename = "prep_time")]
    prep_time: Option<String>,

    #[serde(rename = "photo")]
    photo: Option<String>,

    #[serde(rename = "uid")]
    uid: Option<String>,

    #[serde(rename = "photo_hash")]
    photo_hash: Option<String>,

    #[serde(rename = "created")]
    created: Option<String>,

    #[serde(rename = "source")]
    source: Option<String>,

    #[serde(rename = "rating")]
    rating: Option<i8>,

    #[serde(rename = "ingredients")]
    ingredients: Option<String>,

    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "photo_data")]
    photo_data: Option<String>,

    #[serde(rename = "servings")]
    servings: Option<String>,

    #[serde(rename = "cook_time")]
    cook_time: Option<String>,

    #[serde(rename = "categories")]
    categories: Vec<String>,

    #[serde(rename = "hash")]
    hash: Option<String>,

    #[serde(rename = "directions")]
    directions: Option<String>,

    #[serde(rename = "source_url")]
    source_url: Option<String>,

    #[serde(rename = "notes")]
    notes: Option<String>,

    #[serde(rename = "photo_large")]
    photo_large: Option<serde_json::Value>,
}

impl Recipe {
    pub fn to_md(&self) -> String {
        let mut md = String::new();

        md.push_str(&Md::title(&self.name) );

        if self.description.is_some() {
            md.push_str(self.description.as_ref().unwrap() );
        }
        
        if self.photo_data.is_some() {
            let img_data_uri = format!("data:image/png;base64,{}", self.photo_data.as_ref().unwrap());
            md.push_str(&Md::img(&self.name, &img_data_uri));
        }

        if self.directions.is_some() {

            let directions_block = self.directions.as_ref().unwrap();
            let cleaned_directions_block = directions_block.replace("\n\n", "\n");
            let directions = cleaned_directions_block.split("\n");

            for (index, step) in directions.into_iter().enumerate() {
                md.push_str(&Md::ol(index + 1, step));
            }

        }



        md
    }
}