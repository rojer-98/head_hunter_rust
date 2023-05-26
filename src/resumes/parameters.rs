use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub enum VisibilityListParams {
    WhiteList,
    BlackList,
}

impl Display for VisibilityListParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let vlp = match self {
            Self::WhiteList => "whitelist",
            Self::BlackList => "blacklist",
        };

        write!(f, "{vlp}")
    }
}
