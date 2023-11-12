use super::hevent::Hevent;
use super::hloop::Hloop;

/// ## Idle event
/// The following structures are subclasses of `Hevent`,
/// inheriting `Hevent` data members and function members.
pub struct Hidle<'a> {
    pub hevent: Hevent<'a>,
    pub repeat: u32,

    prev: &'a Hidle<'a>,
    next: &'a Hidle<'a>,
}

impl<'a> Hidle<'a> {
    pub fn new(hloop: &'a Hloop<'a>, prev: &'a Hidle, next: &'a Hidle) -> Self {
        Self {
            hevent: Hevent::new(hloop),
            repeat: 0,
            prev,
            next,
        }
    }
}
