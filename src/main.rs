use std::cell::{RefMut, RefCell};
use std::sync::{RwLock, RwLockWriteGuard};
use std::rc::Rc;

static GLOBAL: RwLock<u8> = RwLock::new(0);

fn _inc_r(r: &RefCell<u8>) {
    *r.borrow_mut() += 1;
}

fn _refcell() {
    let r = RefCell::new(0u8);
    let mut r1: RefMut<u8> = r.borrow_mut();
    *r1 += 1;
    drop(r1);
    let mut r2: RefMut<u8> = r.borrow_mut();
    *r2 += 1;
    drop(r2);
    _inc_r(&r);
    println!("{}", *r.borrow());
}

fn _return_rc() -> Rc<RefCell<u8>> {
    let rc1 = Rc::new(RefCell::new(0u8));
    let rc2 = Rc::clone(&rc1);
    let rc3 = Rc::clone(&rc1);
    *rc1.borrow_mut() += 1;
    *rc2.borrow_mut() += 1;
    rc3
}

fn _rc() {
    let rc = _return_rc();
    println!("{}", *rc.borrow());
}

fn main() {
    let mut g: RwLockWriteGuard<u8> = GLOBAL.write().unwrap();
    *g += 1;
    println!("{}", *g);
}
