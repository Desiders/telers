use std::any::Any;

/// [`Any`] that is [`Clone`] as well, so a boxed value can be cloned without naming its type.
///
/// It is implemented for every `Clone + Send + Sync + 'static` type and makes
/// `Box<dyn AnyClone + Send + Sync>` implement [`Clone`], which is what lets the context and the
/// extensions of a request be cloned together with it.
#[allow(clippy::module_name_repetitions)]
pub trait AnyClone: Any {
    fn clone_box(&self) -> Box<dyn AnyClone + Send + Sync>;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn into_any(self: Box<Self>) -> Box<dyn Any>;
}

impl<T: Clone + Send + Sync + 'static> AnyClone for T {
    fn clone_box(&self) -> Box<dyn AnyClone + Send + Sync> {
        Box::new(self.clone())
    }

    #[inline]
    fn as_any(&self) -> &dyn Any {
        self
    }

    #[inline]
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    #[inline]
    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}

impl Clone for Box<dyn AnyClone + Send + Sync> {
    fn clone(&self) -> Self {
        (**self).clone_box()
    }
}
