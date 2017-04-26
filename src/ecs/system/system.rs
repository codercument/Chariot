// Chariot: An open source reimplementation of Age of Empires (1997)
// Copyright (c) 2016 Kevin Fuller
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use specs;
use super::super::world::SystemGroup;
use types::Fixed;

pub trait System: Send {
    fn update(&mut self, arg: specs::RunArg, time_step: Fixed);
}

pub struct SystemWrapper(Box<System>);

impl SystemWrapper {
    pub fn new(system: Box<System>) -> SystemWrapper {
        SystemWrapper(system)
    }
}

impl specs::System<(SystemGroup, Fixed)> for SystemWrapper {
    fn run(&mut self, arg: specs::RunArg, params: (SystemGroup, Fixed)) {
        match params.0 {
            SystemGroup::Normal => self.0.update(arg, params.1),
            _ => arg.fetch(|_| {}),
        }
    }
}
