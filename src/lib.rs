pub const CM_FIRST: i8 = 1;
pub const CM_LAST: i8 = 2;
pub const CM_RANGE: i8 = 3;

// cut text between start & finish text parts, mode of cutting can be selected
// cm_first will find first, cm_last will search from end, cm_range will cut between first & last
// if only extract and don`t change the input, copyonly must be "true"
pub fn one(start: &str, finish: &str, input: &mut String, mode: i8, copyonly: bool) -> String
{
  let mut original = input.clone();
  let original_static = input.clone();
  
  if input.find(start) == None && start.len() > 0 {
      return String::from("");
  }
  
  let v1: Vec<_> = original_static.match_indices(start).collect();
  let v2: Vec<_> = original_static.match_indices(finish).collect();
  
  let mut read_start = 0;
  let mut read_end = 0;
  
  let mut cut_start = 0;
  let mut cut_end = 0;
  
  if v1.len() > 0 {
      for (index, _) in input.match_indices(start) {
          cut_start = index;
          read_start = index + start.len();
          
          if mode == CM_FIRST || mode == CM_RANGE {
              break;
            }
        }
    }
  
  if v2.len() > 0 {
    for (index, _) in input.match_indices(finish) {
      read_end = index;
      cut_end = index + finish.len();
      if read_end > read_start {
          if mode == CM_FIRST || mode == CM_LAST {
            break;
          }
        }
    }
  }
  
  let mut result = String::from("");
  
  if read_end > read_start && read_end > 0 {
      let mut input_bytes: Vec<u8> = input.bytes().collect();
      let mut result_bytes: Vec<u8> = vec![];
      result_bytes.extend_from_slice(&input_bytes[read_start..read_end]);
      
      result = String::from_utf8(result_bytes).unwrap();
      
      if copyonly == false {
          input_bytes.drain(cut_start..cut_end);
          original = String::from_utf8(input_bytes).unwrap();
        }
    }
  
  input.clear();
  input.push_str(original.as_str());
  return result;
}

// this will cut all appropriate parts
pub fn all(start: &str, finish: &str, input: &mut String, mode: i8, copyonly: bool) -> Vec<String>
{
  let mut original = input.clone();
  let mut result: Vec<String> = vec![];
  let mut work = true;
  
  while work == true {
      let curr = self::one(start, finish, &mut original, mode, copyonly);
      if curr == "" {
        work = false;
      } else {
        result.push(curr);
      }
    }
  input.clear();
  input.push_str(original.as_str());
  return result;
}
