#![crate_type = "dylib"]

#![feature(lang_items, libc)]

extern crate libc;

// Descritpion du module :
//  start : Démarre un compteur de temps
//   idle : Check si le compteur arrive à 10 heures,
//          puis fait un rappel toutes les 15 minutes
//    end : Désactive le module/ le rappel

// 36000 seconds == 10 hours
// 900 seconds == 15 minutes

#[no_mangle]
pub unsafe extern "C" fn start(state: *mut LibraryState, chrono: *mut libc::c_void)
{ if let Some(mut state) = state.as_mut()
  { state.sheet = Sheet::Seiza; //HAPPY
    libc::memcpy(state.message.as_mut_ptr() as *mut libc::c_void, b"Et c'est parti  pour dix heures de travail!".as_ptr() as *const libc::c_void, 43); }
  let the_time = std::time::Instant::now()
  *chrono = (the_time, 36000u32); }

#[no_mangle]
pub unsafe extern "C" fn idle(state: *mut LibraryState, chrono: *mut libc::c_void)
{ let (the_time, checkpoint) = *chrono as (std::time::Instant, u32);
  if the_time.elapsed().as_secs().ge(&checkpoint)
  { if let Some(mut state) = state.as_mut()
    { let repeat_time = std::time::Instant::now()
      state.sheet = Sheet::Bust; //SLEEPY
      libc::memcpy(state.message.as_mut_ptr() as *mut libc::c_void, b"Ca fait 10heuresque tu bosses!  Tu devrais push et arreter pour aujourd'hui!!".as_ptr() as *const libc::c_void, 77);
      *chrono = (repeat_time, 900u32); }}}

#[no_mangle]
pub unsafe extern "C" fn end(state: *mut LibraryState, chrono: *mut libc::c_void)
{ if let Some(mut state) = state.as_mut()
  { state.sheet = Sheet::Seiza; //MISUNDERSTANDING
    libc::memcpy(state.message.as_mut_ptr() as *mut libc::c_void, b"Fait attention ates heures de   sommeil!", 40); }}

/*
Ne      Info-Bulle:
ko:   1234567890123456
--,  ,----------------,
  | 1|                |
  | 2|                |
  | 3|                |
  | 4|                |
  | 5|                |
--'  '----------------'
*/

#[lang = "eh_personality"] extern fn rust_eh_personality() {}
#[lang = "panic_fmt"] extern fn rust_begin_panic() -> ! { loop {} }
