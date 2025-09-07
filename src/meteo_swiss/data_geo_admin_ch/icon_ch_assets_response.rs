use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub struct IconChAssetsResponse {
    pub assets: Vec<IconChAsset>,
}


const HOR_CONSTS_ID: &str = "horizontal_constants_icon-ch1-eps.grib2";
const VERT_CONSTS_ID: &str = "vertical_constants_icon-ch1-eps.grib2";


impl IconChAssetsResponse {
    pub fn find_asset_by_id(&self, id: &str) -> Option<&IconChAsset> {
        self.assets.iter().find(|asset| asset.id == id)
    }
    
    
    pub fn get_horizontal_constants(&self) -> Option<&IconChAsset> {
        self.find_asset_by_id(HOR_CONSTS_ID)
    }
    
    
    pub fn get_vertical_constants(&self) -> Option<&IconChAsset> {
        self.find_asset_by_id(VERT_CONSTS_ID)
    }
}


#[derive(Debug, Deserialize)]
pub struct IconChAsset {
    pub id: String,
    pub title: String,
    pub href: String,
}
