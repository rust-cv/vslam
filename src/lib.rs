use image::GrayImage;

trait FeatureSource<'a> {
    type Out;
    fn extract_features<E>(&'a self, extractor: E) -> Self::Out where E: FeatureExtractor;
}

trait FeatureExtractor {
}
