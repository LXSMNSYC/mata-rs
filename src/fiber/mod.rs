/**
 * @license
 * MIT License
 *
 * Copyright (c) 2020 Alexis Munsayac
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 *
 * @author Alexis Munsayac <alexis.munsayac@gmail.com>
 * @copyright Alexis Munsayac 2020
 */
use crate::components::Fundamental;
use crate::utils::tags::{
  Work,
  Type,
};
use crate::hooks::Hook;
use std::error::Error;

mod begin_work;
mod error_registry;

pub struct Fiber {
  /**
   * Fiber types
   */
  work_type: Work,
  component_type: Type,
  /**
   * Reference to the node
   */
  component_node: Option<&'static dyn Fundamental>,
  /**
   * Fiber links
   */
  parent: Option<&'static Fiber>,
  child: Option<&'static Fiber>,
  sibling: Option<&'static Fiber>,
  alternate: Option<&'static Fiber>,
  /**
   * Fiber indexing
   */
  index: Option<i32>,
  key: Option<String>,
  /**
   * Error registry
   */
  errors: Vec<&'static dyn Error>,
  /**
   * Hook registry
   */
  hooks: Vec<&'static dyn Hook>,
}

impl Fiber {
  fn new() -> Fiber {
    return Fiber {
      work_type: Work::NoWork,
      component_type: Type::NoType,
      component_node: None,
      parent: None,
      child: None,
      sibling: None,
      alternate: None,
      index: None,
      key: None,
      errors: vec![],
      hooks: vec![],
    };
  }
}