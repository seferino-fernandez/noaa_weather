use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TextProductTypeCollectionGraphInner {
    #[serde(rename = "productCode")]
    pub product_code: String,
    #[serde(rename = "productName")]
    pub product_name: String,
}

impl TextProductTypeCollectionGraphInner {
    pub fn new(product_code: String, product_name: String) -> TextProductTypeCollectionGraphInner {
        TextProductTypeCollectionGraphInner {
            product_code,
            product_name,
        }
    }
}
