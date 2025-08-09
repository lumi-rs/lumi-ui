use num_traits::AsPrimitive;

pub enum EasingFunction<T: CustomEasingFunction = ()> {
    Linear,
    // Other easing functions here
    Custom(T)
}

pub trait CustomEasingFunction {
    fn calculate<U, V>(&self, start: &U, end: &U, progress: f32) -> V;
}

impl<T: CustomEasingFunction> EasingFunction<T> {
    pub fn calculate<U: AsPrimitive<f32>, V: Copy + 'static>(&self, start: &U, end: &U, progress: f32) -> V where f32: AsPrimitive<V> {
        let difference = end.as_() - start.as_();
        let result = match self {
            EasingFunction::Linear => start.as_() + (difference * progress),
            // implementation here
            Self::Custom(function) => {
                return function.calculate(start, end, progress)
            }
        };

        result.as_()
    }
}

impl CustomEasingFunction for () {
    fn calculate<U, V>(&self, _start: &U, _end: &U, _progress: f32) -> V {
        unimplemented!("Please Provide a custom type for EasingFunction::Custom!")
    }
}