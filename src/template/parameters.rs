use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub enum TemplateParams {
    Template,
    Template1,
}

impl Display for TemplateParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let t = match self {
            Self::Template => "Template",
            Self::Template1 => "Template1",
        };

        write!(f, "{t}")
    }
}
