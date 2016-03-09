extern crate libc;
use std::ffi::{CStr};

#[derive(PartialEq)]
enum ParseState {
    Word,
    LineBreak,
    HtmlTagOpen,
    HtmlTag,
    HtmlTagClose,
    Space
}

static MAX_PARAGRAPH_COUNT:usize = 100;

#[repr(C)]
pub struct Array {
    len: libc::size_t,
    data: *const libc::c_void,
}

impl Array {
  fn from_vec<T>(mut vec: Vec<T>) -> Array {
    vec.shrink_to_fit();
    let array = Array { data: vec.as_ptr() as *const libc::c_void, len: vec.len() as libc::size_t };
    std::mem::forget(vec);
    array
  }
}

#[no_mangle]
pub extern fn word_counts(s1: *const libc::c_char) -> Array {
  let s1_cstr = unsafe { CStr::from_ptr(s1) };
  let s1_and_str = s1_cstr.to_str().unwrap();
  let body = s1_and_str.to_string();

  let counts = word_counts_internal(&body);
  return Array::from_vec(counts);
}

// #[no_mangle]
// pub extern fn word_counts_free(s: *mut libc::c_char) {
//   unsafe { CString::from_raw(s); }
// }

fn word_counts_internal(body: &String) -> Vec<u16> {
  let mut counts:Vec<u16> = Vec::with_capacity(MAX_PARAGRAPH_COUNT);
  let mut state:ParseState = ParseState::Space;
  let mut current_count = 0_u16;

  for c in body.chars() {
    if (state == ParseState::HtmlTagOpen || state == ParseState::HtmlTag) && c != '>' {
      if c == 'p' || c == 'b' || c == 'd' {
        if current_count > 0 {
          counts.push(current_count);
        }
        current_count = 0;
        if counts.len() > MAX_PARAGRAPH_COUNT {
          break;
        }
        state = ParseState::HtmlTag;
      }
      continue;
    }

    if c == '\n' {
        match state {
            ParseState::Word => current_count += 1,
            ParseState::LineBreak => {
              if current_count > 0 {
                counts.push(current_count);
              }
              current_count = 0;
              if counts.len() > MAX_PARAGRAPH_COUNT {
                break;
              }
            },
            _ => {}
        }
        state = ParseState::LineBreak;
    }
    else if c == ' ' {
        if state == ParseState::Word {
          current_count += 1;
        }
        state = ParseState::Space;
    }
    else if c == '<' {
      if state == ParseState::Word {
        current_count += 1;
      }
      state = ParseState::HtmlTagOpen;
    }
    else if c == '>' {
      state = ParseState::HtmlTagClose;
    }
    else {
      state = ParseState::Word;
    }
  }

  if state == ParseState::Word {
    current_count += 1;
  }

  if current_count > 0 {
    counts.push(current_count);
  }

  return counts;
}

#[test]
fn simple() {
  let body = "one two three four".to_string();
  assert_eq!(word_counts_internal(&body), vec![4]);

  let body = "one two\n\nthree four".to_string();
  assert_eq!(word_counts_internal(&body), vec![2,2]);
}

#[test]
fn linebreak() {
  let body = "one two\n\nthree four".to_string();
  assert_eq!(word_counts_internal(&body), vec![2,2]);
}

#[test]
fn unformatted_html() {
  let body = "<p>one two</p><p>one two three four</p>".to_string();
  assert_eq!(word_counts_internal(&body), vec![2,4]);
}

#[test]
fn multiple_paragraphs() {
  let body = "
        one two three four five six seven eight nine ten

        one two

        one two three four
".to_string();
  assert_eq!(word_counts_internal(&body), vec![10,2,4]);
}

#[test]
fn mult_br_before() {
  let body = "
one two three four five six seven eight nine ten<br><br><p>one two</p>

<p>one two three four</p>
".to_string();
  assert_eq!(word_counts_internal(&body), vec![10,2,4]);
}

#[test]
fn one_word() {
  let body = "one".to_string();
  assert_eq!(word_counts_internal(&body), vec![1]);
}
