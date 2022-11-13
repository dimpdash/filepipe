use derive_new::new;

use super::Transformer;

#[derive(new)]
pub struct NonInteractiveTransformer {
    name: String,
}

impl NonInteractiveTransformer {}

impl Transformer for NonInteractiveTransformer {}
