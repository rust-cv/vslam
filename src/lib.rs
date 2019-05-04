struct Features<K, D> {
    pub keypoints: K,
    pub descriptors: D,
}

trait FeatureExtractor<'a, Source> {
    type Features: 'a;

    fn extract(&self, source: &'a Source) -> Self::Features;
}

trait KeypointExtractor<'a, Source> {
    type Keypoints: 'a;

    fn extract_keypoints(&self, source: &'a Source) -> Self::Keypoints;
}

trait DescriptorExtractor<'a, Source, Keypoints> {
    type Descriptors: 'a;

    fn extract_descriptors(&self, source: &'a Source, keypoints: &Keypoints) -> Self::Descriptors;
}

pub struct KeypointDescriptorExtractor<K, D>(pub K, pub D);

impl<'a, Source, Keypoints: 'a, K, D> FeatureExtractor<'a, Source>
    for KeypointDescriptorExtractor<K, D>
where
    K: KeypointExtractor<'a, Source, Keypoints = Keypoints>,
    D: DescriptorExtractor<'a, Source, Keypoints>,
{
    type Features = Features<Keypoints, D::Descriptors>;

    fn extract(&self, source: &'a Source) -> Self::Features {
        let keypoints = self.0.extract_keypoints(source);
        let descriptors = self.1.extract_descriptors(source, &keypoints);
        Features {
            keypoints,
            descriptors,
        }
    }
}
