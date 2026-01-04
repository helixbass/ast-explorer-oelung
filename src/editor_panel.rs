use oelung::{anyhow, Component, ComponentInterface, Grid};

pub struct EditorPanel<'a> {
    pub source_text: &'a str,
}

impl<'a> EditorPanel<'a> {
    pub fn new(source_text: &'a str) -> Self {
        Self { source_text }
    }
}

impl<'a> ComponentInterface for EditorPanel<'a> {
    fn render(&self, _grid: Grid) -> Result<Component<'_>, anyhow::Error> {
        unimplemented!()
    }

    fn flex_grow(&self) -> Option<f64> {
        Some(1.0)
    }
}
