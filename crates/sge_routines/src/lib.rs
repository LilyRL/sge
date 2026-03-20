use global::global;
use slotmap::{SlotMap, new_key_type};

new_key_type! {pub struct RoutineKey;}

global!(Routines, routines);

pub struct Routines {
    end_of_this_frame: SlotMap<RoutineKey, Routine>,
    end_of_frame: SlotMap<RoutineKey, Routine>,
}

pub fn init() {
    set_routines(Routines {
        end_of_this_frame: SlotMap::with_key(),
        end_of_frame: SlotMap::with_key(),
    });
}

pub fn update() {
    get_routines().end_of_frame();
}

impl Routines {
    pub fn register_end_of_this_frame(&mut self, routine: Routine) -> RoutineKey {
        self.end_of_this_frame.insert(routine)
    }

    pub fn register_end_of_frame(&mut self, f: impl Fn(usize) + 'static, ptr: usize) -> RoutineKey {
        self.end_of_frame.insert(Routine {
            ptr,
            f: Box::new(f),
        })
    }

    pub fn end_of_frame(&mut self) {
        for routine in self.end_of_this_frame.values() {
            routine.run();
        }

        self.end_of_this_frame.retain(|_, routine| {
            routine.run();
            false
        });
    }

    pub fn deregister(&mut self, key: RoutineKey) {
        self.end_of_this_frame.remove(key);
        self.end_of_frame.remove(key);
    }
}

pub struct Routine {
    ptr: usize,
    f: Box<dyn Fn(usize)>,
}

impl Routine {
    pub fn run(&self) {
        (self.f)(self.ptr);
    }
}
