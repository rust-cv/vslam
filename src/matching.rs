pub trait FeatureMatcher<Features> {
    type Matches;

    fn matching(&self, features: (Features, Features)) -> Self::Matches;
}