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
/**
 * Size of the allocator
 */
static INITIAL_MEMORY_SIZE: usize = 8192usize;

pub struct Alloc {
  stack: Vec<i32>,
  valid: Vec<bool>,
}

impl Alloc {
  pub fn new() -> Self {
    let mut stack: Vec<i32> = Vec::with_capacity(INITIAL_MEMORY_SIZE);
    let valid: Vec<bool> = Vec::with_capacity(INITIAL_MEMORY_SIZE);

    stack.push(1);

    Alloc {
      stack: stack,
      valid: valid,
    }
  }

  fn get(&mut self) -> i32 {
    match self.stack.pop() {
      Some(index) => index,
      None => 0,
    }
  }

  /**
   * Checks if the given index is an allocated index from
   * the Alloc instance.
   */
  pub fn is_valid(&mut self, index: i32) -> bool {
    let loc = index as usize;
    if loc >= self.valid.len() {
      return false;
    }
    match self.valid.get(loc) {
      Some(result) => *result,
      None => false,
    }
  }

  /**
   * Allocates an index.
   */
  pub fn alloc(&mut self) -> i32 {
    let bottom = self.get();

    if self.stack.last() == None {
      self.stack.push(bottom + 1);
      self.valid.resize((bottom + 1) as usize, false);
    }

    self.valid[bottom as usize] = true;

    return bottom;
  }

  /**
   * Frees the allocated index. Returns true if the index
   * is a valid index.
   */
  pub fn free(&mut self, index: i32) -> bool {
    if self.is_valid(index) {
      self.stack.push(index);
      self.valid[index as usize] = false;
      return true;
    }
    return false;
  }
}