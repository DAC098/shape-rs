pub trait MapShape {

    fn map_shape(&mut self, rhs: Self);
    
}

#[inline]
pub fn assign_map_value<T>(lhs: &mut Option<T>, rhs: Option<T>) {
    if let Some(rhs_value) = rhs { lhs.replace(rhs_value); }
}

#[inline]
pub fn assign_map_struct<T>(lhs: &mut Option<T>, rhs: Option<T>) 
where
    T: MapShape
{
    if let Some(lhs_value) = lhs.as_mut() {
        if let Some(rhs_value) = rhs {
            MapShape::map_shape(lhs_value, rhs_value);
        }
    } else if let Some(rhs_value) = rhs {
        lhs.replace(rhs_value);
    }
}
