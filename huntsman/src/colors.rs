//! Provides some color shorthands.

pub use huntsman_comm::RGB;

/// Try to convert a named color into a color struct.
pub fn str_to_color(v: &str) -> Option<RGB>
{
    // Primaries
    if v == "red"
    {
        return Some(RGB{r: 255, g: 0, b: 0});
    }
    if v == "green"
    {
        return Some(RGB{r: 0, g: 255, b: 0});
    }
    if v == "blue"
    {
        return Some(RGB{r: 0, g: 0, b: 255});
    }

    // Secondaries
    if v == "magenta"
    {
        return Some(RGB{r: 255, g: 0, b: 255});
    }
    if v == "yellow"
    {
        return Some(RGB{r: 255, g: 255, b: 0});
    }
    if v == "cyan"
    {
        return Some(RGB{r: 0, g: 255, b: 255});
    }

    // Saturation extremities.
    if v == "black"
    {
        return Some(RGB{r: 0, g: 0, b: 0});
    }
    if v == "white"
    {
        return Some(RGB{r: 255, g: 255, b: 255});
    }

    None
}
