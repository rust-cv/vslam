pub trait FeatureMatcher<Keypoints> {
    type Matches;

    fn matching(&self, keypoints: (Keypoints, Keypoints)) -> Self::Matches;
}