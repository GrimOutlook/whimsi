use crate::tables::component::helper::Component;

pub(crate) trait HasComponent {
    fn component(&self) -> &Component;
    fn component_mut(&mut self) -> &mut Component;
}

#[macro_export]
macro_rules! has_component_boilerplate {
    () => {
        fn component(&self) -> &Component {
            &self.component
        }

        fn component_mut(&mut self) -> &mut Component {
            &mut self.component
        }
    };
}
