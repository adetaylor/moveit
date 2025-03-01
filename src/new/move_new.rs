// Copyright 2021 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use core::mem::MaybeUninit;
use core::pin::Pin;

use crate::move_ref::DerefMove;
use crate::move_ref::MoveRef;
use crate::move_ref::PinExt as _;
use crate::new;
use crate::new::New;

/// A move constructible type: a destination-aware `Clone` that destroys the
/// moved-from value.
///
/// # Safety
///
/// After [`MoveNew::move_new()`] is called:
/// - `src` should be treated as having been destroyed.
/// - `this` must have been initialized.
pub unsafe trait MoveNew: Sized {
  /// Move-construct `src` into `this`, effectively re-pinning it at a new
  /// location.
  ///
  /// # Safety
  ///
  /// The same safety requirements of [`New::new()`] apply, but, in addition,
  /// `*src` must not be used after this function is called, because it has
  /// effectively been destroyed.
  unsafe fn move_new(
    src: Pin<MoveRef<Self>>,
    this: Pin<&mut MaybeUninit<Self>>,
  );
}

/// Returns a [`New`] that forwards to [`MoveNew`].
#[inline]
pub fn mov<P>(ptr: impl Into<Pin<P>>) -> impl New<Output = P::Target>
where
  P: DerefMove,
  P::Target: MoveNew,
{
  let ptr = ptr.into();
  unsafe {
    new::by_raw(move |this| {
      crate::moveit!(let ptr = &move ptr);
      MoveNew::move_new(Pin::as_move(ptr), this);
    })
  }
}
