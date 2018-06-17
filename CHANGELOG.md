<a name=""></a>
##  (2018-06-17)


#### Features

* **OwningRef:**  Add OwningRef trait ([9a85cc62](9a85cc62))
* **alarm-base:**  add LockedAlloc and impls for all mutex allocators ([dd158100](dd158100))
* **base:**  Add alarm-base crate ([ae323685](ae323685))
* **base::Lend:**  Start porting Lend API from sos-alloc (#24) ([60549854](60549854))
* **base::frame::Allocator:**  First pass on frame allocator trait (#22) ([3e507261](3e507261))
* **intruder_alarm:**  Bring back `OwningRef` impl for`&'a T` ([130023b6](130023b6))
* **intruder_alarm::List:**
  *  Add peek methods (#23) ([a5995f4e](a5995f4e), closes [#10](10))
  *  Add trait impls for `List`s of `UnsafeRef`s ([a1531875](a1531875))
  *  Add immutable cursor ([d25b3070](d25b3070))
  *  Add Extend/FromIter & tests ([1dcb155e](1dcb155e))
  *  Implement List::pop ([140e39b8](140e39b8))
  *  Implement List::push ([79b0af1b](79b0af1b))
* **intruder_alarm::UnsafeRef:**
  *  impl `Clone` for `UnsafeRef` ([41a3b9d8](41a3b9d8))
  *  add `PartialEq` impl for `UnsafeRef` ([8cc0c36e](8cc0c36e))
  *  add `UnsafeRef::from_box` to aid testing ([bfd221b1](bfd221b1))
  *  add UnsafeRef for unsafe intrusive collections ([903f5ce3](903f5ce3))
* **intruder_alarm::cursor:**
  *  add `IntoCursor` trait ([256d3714](256d3714))
  *  add `Cursor` and`CursorMut` traits ([044e2b66](044e2b66))
  *  Add `map_in_place` to `CursorMut` ([576988bd](576988bd))
  *  Add CursorMut API ([d08ccd5a](d08ccd5a))
* **intruder_alarm::list::CursorMut:** Implement `remove` ([e91f9a81](e91f9a81))
* **intruder_alarm::stack:**
  *  add intrusive singly-linked list (#35) ([2b0614ae](2b0614ae), closes [#8](8))
  *  UnsafeRef impl (#44) ([d520b1b8](d520b1b8))

#### Bug Fixes

*   replace uses of `ptr::Shared` with `ptr::NonNull` (#20) (#25) ([7508dfc9](7508dfc9))
* **base:**  Update alloc API to use ptr::NonNull ([22e46b8a](22e46b8a))
* **intruder_alarm:**
  *  fix build with `std` feature flag ([e8004df3](e8004df3), closes [#26](26))
  *  fix incorrect cfg_attr syntax ([314969f1](314969f1))
* **intruder_alarm::cursor:**  fix iterator impl for cursor (#40) ([68f392bc](68f392bc))
* **slabby:**  require `ptr_internals` feature to build on nightly (#16) ([e73e774a](e73e774a), closes [#14](14))




