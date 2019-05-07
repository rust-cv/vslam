pub struct KeypointDescriptor<K, D> {
    pub keypoints: K,
    pub descriptors: D,
}

pub trait FeatureExtractor<'a, Source> {
    type Features: 'a;

    fn extract(&self, source: &'a Source) -> Self::Features;
}

pub trait KeypointDetector<'a, Source> {
    type Keypoints: 'a;

    fn detect_keypoints(&self, source: &'a Source) -> Self::Keypoints;
}

pub trait DescriptorExtractor<'a, Source, Keypoints> {
    type Features: 'a;

    fn extract_descriptors(&self, source: &'a Source, keypoints: Keypoints) -> Self::Features;
}

pub struct KeypointDetectorExtractor<K, D>(pub K, pub D);

impl<'a, Source, Keypoints: 'a, K, D> FeatureExtractor<'a, Source>
    for KeypointDetectorExtractor<K, D>
where
    K: KeypointDetector<'a, Source, Keypoints = Keypoints>,
    D: DescriptorExtractor<'a, Source, Keypoints>,
{
    type Features = D::Features;

    fn extract(&self, source: &'a Source) -> Self::Features {
        let keypoints = self.0.detect_keypoints(source);
        self.1.extract_descriptors(source, keypoints)
    }
}
