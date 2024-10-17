use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

trait RoadCrosser {
    fn cross_road(&self) -> Result<(), Box<dyn Any>>;
    fn state(&self) -> &RefCell<Rc<Box<dyn RoadCrosserState>>>;
    fn look(&self, dir: Direction);
    fn change_state(&self, state: Rc<Box<dyn RoadCrosserState>>);
}

enum Direction {
    Left,
    Right,
}

trait RoadCrosserState {
    fn cross_road(&self, road_crosser: &dyn RoadCrosser) -> Result<(), Box<dyn Any>>;
    fn look(&self, road_crosser: &dyn RoadCrosser, dir: Direction);
}

struct UnawareCrosser {
    lefts_left: RefCell<i32>,
    rights_left: RefCell<i32>,
}

impl UnawareCrosser {
    fn new() -> Box<dyn RoadCrosserState> {
        Box::new(UnawareCrosser {
            lefts_left: RefCell::new(2),
            rights_left: RefCell::new(1),
        })
    }
}

impl RoadCrosserState for UnawareCrosser {
    fn cross_road(&self, road_crosser: &dyn RoadCrosser) -> Result<(), Box<dyn Any>> {
        Err(Box::new("You need to look both ways first".to_string()))
    }

    fn look(&self, road_crosser: &dyn RoadCrosser, dir: Direction) {
        match dir {
            Direction::Left => {
                *self.lefts_left.borrow_mut() -= 1;
            },
            Direction::Right => {
                *self.rights_left.borrow_mut() -= 1;
            },
        }
        if *self.lefts_left.borrow() <= 0 && *self.rights_left.borrow() <= 0 {
            road_crosser.change_state(Rc::new(AwareCrosser::new()));
        }
    }
}

struct AwareCrosser {
}

impl AwareCrosser {
    fn new() -> Box<dyn RoadCrosserState> {
        Box::new(AwareCrosser {
        })
    }
}

impl RoadCrosserState for AwareCrosser {
    fn cross_road(&self, road_crosser: &dyn RoadCrosser) -> Result<(), Box<dyn Any>> {
        Ok(())
    }

    fn look(&self, road_crosser: &dyn RoadCrosser, dir: Direction) {
        println!("The time for looking is in the past. Now it's time for crossing");
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

    fn cross_road(&self) -> Result<(), Box<dyn Any>> {
        let state = Rc::clone(&self.state().borrow());
        state.cross_road(self)
    }

    fn look(&self, dir: Direction) {
        let state = Rc::clone(&self.state().borrow());
        state.look(self, dir);
    }

    fn change_state(&self, state: Rc<Box<dyn RoadCrosserState>>) {
        *self.state().borrow_mut() = state;
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

    #[test]
    fn aware_chicken_should_cross_the_road() {
        // given
        let chicken = Chicken::new();

        // when
        chicken.look(Direction::Left);
        chicken.look(Direction::Right);
        chicken.look(Direction::Left);
        let result = chicken.cross_road();

        // then
        assert!(result.is_ok());
    }
}
