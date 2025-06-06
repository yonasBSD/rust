#![allow(unused)]
#![allow(clippy::struct_field_names)]
#![warn(clippy::misnamed_getters)]

struct A {
    a: u8,
    b: u8,
    c: u8,
}

impl A {
    fn a(&self) -> &u8 {
        //~^ misnamed_getters

        &self.b
    }
    fn a_mut(&mut self) -> &mut u8 {
        //~^ misnamed_getters

        &mut self.b
    }

    fn b(self) -> u8 {
        //~^ misnamed_getters

        self.a
    }

    fn b_mut(&mut self) -> &mut u8 {
        //~^ misnamed_getters

        &mut self.a
    }

    fn c(&self) -> &u8 {
        //~^ misnamed_getters

        &self.b
    }

    fn c_mut(&mut self) -> &mut u8 {
        //~^ misnamed_getters

        &mut self.a
    }
}

union B {
    a: u8,
    b: u8,
}

impl B {
    unsafe fn a(&self) -> &u8 {
        //~^ misnamed_getters

        unsafe { &self.b }
    }
    unsafe fn a_mut(&mut self) -> &mut u8 {
        //~^ misnamed_getters

        unsafe { &mut self.b }
    }

    unsafe fn b(self) -> u8 {
        //~^ misnamed_getters

        unsafe { self.a }
    }

    unsafe fn b_mut(&mut self) -> &mut u8 {
        //~^ misnamed_getters

        unsafe { &mut self.a }
    }

    unsafe fn c(&self) -> &u8 {
        unsafe { &self.b }
    }

    unsafe fn c_mut(&mut self) -> &mut u8 {
        unsafe { &mut self.a }
    }

    unsafe fn a_unchecked(&self) -> &u8 {
        //~^ misnamed_getters

        unsafe { &self.b }
    }
    unsafe fn a_unchecked_mut(&mut self) -> &mut u8 {
        //~^ misnamed_getters

        unsafe { &mut self.b }
    }

    unsafe fn b_unchecked(self) -> u8 {
        //~^ misnamed_getters

        unsafe { self.a }
    }

    unsafe fn b_unchecked_mut(&mut self) -> &mut u8 {
        //~^ misnamed_getters

        unsafe { &mut self.a }
    }

    unsafe fn c_unchecked(&self) -> &u8 {
        unsafe { &self.b }
    }

    unsafe fn c_unchecked_mut(&mut self) -> &mut u8 {
        unsafe { &mut self.a }
    }
}

struct D {
    d: u8,
    inner: A,
}

impl core::ops::Deref for D {
    type Target = A;
    fn deref(&self) -> &A {
        &self.inner
    }
}

impl core::ops::DerefMut for D {
    fn deref_mut(&mut self) -> &mut A {
        &mut self.inner
    }
}

impl D {
    fn a(&self) -> &u8 {
        //~^ misnamed_getters

        &self.b
    }
    fn a_mut(&mut self) -> &mut u8 {
        //~^ misnamed_getters

        &mut self.b
    }

    fn d(&self) -> &u8 {
        //~^ misnamed_getters

        &self.b
    }
    fn d_mut(&mut self) -> &mut u8 {
        //~^ misnamed_getters

        &mut self.b
    }
}

fn main() {
    // test code goes here
}
