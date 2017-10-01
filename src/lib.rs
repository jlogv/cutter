pub const CM_FIRST: i8 = 1;
pub const CM_LAST: i8 = 2;
pub const CM_RANGE: i8 = 3;

pub fn one(start: &str, finish: &str, input: &mut String, cm: i8, copyonly: bool) -> String
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
          
          if cm == CM_FIRST || cm == CM_RANGE {
              break;
            }
        }
    }
  
  if v2.len() > 0 {
    for (index, _) in input.match_indices(finish) {
      read_end = index;
      cut_end = index + finish.len();
      if read_end > read_start {
          if cm == CM_FIRST || cm == CM_LAST {
            break;
          }
        }
    }
  }
  
  let mut result = String::from("");
  
  if read_end > read_start && read_end > 0 {
      let mut ccc: Vec<u8> = input.bytes().collect();
      let mut ddd: Vec<u8> = vec![];
      ddd.extend_from_slice(&ccc[read_start..read_end]);
      
      result = String::from_utf8(ddd).unwrap();
      
      if copyonly == false {
          ccc.drain(cut_start..cut_end);
          original = String::from_utf8(ccc).unwrap();
        }
    }
  
  input.clear();
  input.push_str(original.as_str());
  return result;
}

pub fn all(start: &str, finish: &str, input: &mut String, cm: i8, copyonly: bool/*, output : &mut String*/) -> Vec<String>
{
  let mut original = input.clone();
  let mut result: Vec<String> = vec![];
  let mut work = true;
  
  while work == true {
      let curr = self::one(start, finish, &mut original, cm, copyonly);
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
