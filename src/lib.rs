use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

trait RoadCrosser {
    fn cross_road(&self) -> Result<(), Box<dyn Any>>;
    /*
    fn cross_road(&self) -> Result<(), Box<dyn Any>> {
        let state = Rc::clone(&self.state().borrow());
        state.cross_road(self)
    }
    */
    /*
    fn cross_road(self: Box<Self>) -> Result<(), Box<dyn Any>> {
        let state = Rc::clone(&self.state().borrow());
        state.cross_road(self)
    }
    */
    fn state(&self) -> &RefCell<Rc<Box<dyn RoadCrosserState>>>;
}

trait RoadCrosserState {
    //fn cross_road(&self, road_crosser: &dyn RoadCrosser) -> Result<(), Box<dyn Any>>;
    fn cross_road(&self, road_crosser: Box<dyn RoadCrosser>) -> Result<(), Box<dyn Any>>;
}

struct UnawareCrosser {
}

impl UnawareCrosser {
    fn new() -> Box<dyn RoadCrosserState> {
        Box::new(UnawareCrosser {
        })
    }
}

impl RoadCrosserState for UnawareCrosser {
    //fn cross_road(&self, road_crosser: &dyn RoadCrosser) -> Result<(), Box<dyn Any>> {
    fn cross_road(&self, road_crosser: Box<dyn RoadCrosser>) -> Result<(), Box<dyn Any>> {
        Ok(())
    }
}

struct Chicken {
    state: RefCell<Rc<Box<dyn RoadCrosserState>>>,
}

impl Chicken {
    fn new() -> Box<dyn RoadCrosser> {
        Box::new(Chicken {
            state: RefCell::new(Rc::new(UnawareCrosser::new())),
        })
    }
}

impl RoadCrosser for Chicken {
    fn state(&self) -> &RefCell<Rc<Box<dyn RoadCrosserState>>> {
        &self.state
    }

    fn cross_road(self: &Box<dyn RoadCrosser>) -> Result<(), Box<dyn Any>> {
        let state = Rc::clone(&self.state().borrow());
        state.cross_road(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chicken_should_not_cross_the_road_without_looking() {
        // given
        let chicken = Chicken::new();

        // when
        let result = chicken.cross_road();

        // then
        assert!(result.is_err());
    }
}
