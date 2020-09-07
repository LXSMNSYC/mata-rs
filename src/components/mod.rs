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
use crate::utils::tags::{
  Type,
};

/**
 * Base of all components
 */
pub trait Fundamental {
  /**
   * The render logic of a Fundamental
   */
  fn render(&self) -> Option<&dyn Fundamental>;

  /**
   * Indicates the kind of the node, used by fibers to
   * apply the specified way of rendering.
   */
  fn get_type(&self) -> Type;

  /**
   * Gets the children nodes. Usually through struct properties
   * that receives the nodes.
   */
  fn get_children(&self) -> Option<Vec<&dyn Fundamental>>;

  /**
   * Gets the key of the node, used for implying node uniqueness.
   */
  fn get_key(&self) -> Option<String>;
}

/**
 * A Fragment is a kind of component that allows renderable components
 * to return multiple child components at the same level.
 */
pub struct Fragment {
  key: Option<String>,
  /**
   * A Vector of component nodes
   */
  children: Vec<&'static dyn Fundamental>,
}

impl Fundamental for Fragment {
  /**
   * A BasicComponent may optionally return a Fundamental.
   */
  fn render(&self) -> Option<&dyn Fundamental> {
    return None;
  }
  /**
   * Fragment is a special type of Component
   */
  fn get_type(&self) -> Type {
    return Type::Fragment;
  }
  /**
   * Since Fragment isn't a renderable component, we might
   * need to refer to the children property for reconciliation.
   */
  fn get_children(&self) -> Option<Vec<&dyn Fundamental>> {
    return Some(self.children);
  }
  /**
   * Return the key from the property.
   */
  fn get_key(&self) -> Option<String> {
    return self.key.clone();
  }
}

/**
 * A BasicComponent is a kind of Fundamental that has no access to hooks, which
 * implies that this component has no way of keeping component-level data or
 * lifecycle handling.
 */
pub trait BasicComponent : Fundamental {
  fn get_type(&self) -> Type {
    return Type::Basic;
  }

  /**
   * A BasicComponent does not have a children coming from
   * any source besides the render method.
   */
  fn get_children(&self) -> Option<Vec<&dyn Fundamental>> {
    return None;
  }

  /**
   * Gets the key of the node, used for implying node uniqueness.
   */
  fn get_key(&self) -> Option<String>;

  /**
   * The render logic of a Fundamental
   */
  fn render(&self) -> Option<&dyn Fundamental>;
}

/**
 * A Component is a kind of RenderableComponent that serves as the fundamental and core
 * Component implementation. Component allows you to have access to hooks, which implies
 * access to component-level data persistence and lifecycles.
 */
pub trait Component : Fundamental {
  fn get_type(&self) -> Type {
    return Type::Component;
  }

  /**
   * A Component does not have a children coming from
   * any source besides the render method.
   */
  fn get_children(&self) -> Option<Vec<&dyn Fundamental>> {
    return None;
  }

  /**
   * Gets the key of the node, used for implying node uniqueness.
   */
  fn get_key(&self) -> Option<String>;
  /**
   * The render logic of a Fundamental
   */
  fn render(&self) -> Option<&dyn Fundamental>;
}
